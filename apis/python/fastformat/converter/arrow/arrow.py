import pyarrow as pa

from typing import List

class ArrowDataBuilder:
    def __init__(self):
        self.children = []
        self.field_names = []

    def push(self, child: pa.Array, field_name: str):
        self.children.append(child)
        self.field_names.append(field_name)

    def build(self) -> pa.UnionArray:
        return pa.UnionArray.from_dense(
            types=pa.array([], type=pa.int8()),
            children=self.children,
            field_names=self.field_names,
            value_offsets=pa.array([], type=pa.int32())
        )

class ArrowDataViewer:
    def __init__(self, data: pa.UnionArray):
        names = [field.name for field in data.type]
        indices = [index for index in data.type.type_codes]

        self.data = data
        self.map = dict(zip(names, indices))

    def primitive_singleton(self, name: str):
        if name not in self.map:
            raise KeyError(f'Field {name} not found in data')

        return self.data.field(self.map[name]).to_numpy()[0]

    def utf8_singleton(self, name: str):
        if name not in self.map:
            raise KeyError(f'Field {name} not found in data')

        return self.data.field(self.map[name]).to_numpy(zero_copy_only=False)[0]

    def primitive_array(self, name: str):
        if name not in self.map:
            raise KeyError(f'Field {name} not found in data')

        return self.data.field(self.map[name]).to_numpy()

    def utf8_array(self, name: str):
        if name not in self.map:
            raise KeyError(f'Field {name} not found in data')

        return self.data.field(self.map[name]).to_pylist()
