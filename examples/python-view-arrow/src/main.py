import pyarrow as pa
import numpy as np

from dataclasses import dataclass

from fastformat.converter.arrow import into_arrow, ArrowViewer

@dataclass
class CustomDataType:
    size: np.uint32
    label: str
    ranges: np.ndarray

    def into_arrow(self) -> pa.UnionArray:
        return into_arrow(
            children=[
                pa.array([self.size]),
                pa.array([self.label]),
                pa.array(self.ranges)
            ],
            field_names=['size', 'label', 'ranges'])

    @staticmethod
    def from_arrow(data: pa.UnionArray):
        viewer = ArrowViewer(data)

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
