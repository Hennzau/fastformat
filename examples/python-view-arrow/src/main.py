import pyarrow as pa
import numpy as np

from dataclasses import dataclass

@dataclass
class CustomDataType:
    size: np.uint32
    label: str
    ranges: np.ndarray

    def into_arrow(self) -> pa.UnionArray:
        from fastformat.converter.arrow import ArrowDataBuilder

        builder = ArrowDataBuilder()

        builder.push(pa.array([self.size]), 'size')
        builder.push(pa.array([self.label]), 'label')
        builder.push(pa.array(self.ranges), 'ranges')

        return builder.build()

    @staticmethod
    def from_arrow(data: pa.UnionArray):
        from fastformat.converter.arrow import ArrowDataViewer

        viewer = ArrowDataViewer(data)

        return CustomDataType(
            size=viewer.primitive_singleton('size'),
            label=viewer.utf8_singleton('label'),
            ranges=viewer.primitive_array('ranges')
        )

custom_data = CustomDataType(
    size=np.uint32(42),
    label='custom',
    ranges=np.array([1, 2, 3], dtype=np.uint32)
)

arrow_data = custom_data.into_arrow()
reconstructed_data = CustomDataType.from_arrow(arrow_data)

print(reconstructed_data)
