import pyarrow as pa

from typing import List, Self

class ArrowDataBuilder:
    def __init__(self):
        ...

    def push(self, child: pa.Array, field_name: str):
        ...

    def build(self) -> pa.UnionArray:
        ...

class ArrowDataViewer:
    def __init__(self, data: pa.UnionArray):
        """
        Initialize an ArrowViewer with a UnionArray. It reads the array and finds the indices of the fields.
        """
        ...

    def primitive_singleton(self, name: str):
        """
        Retrieve a single primitive value from the UnionArray. The value is cloned
        """
        ...

    def utf8_singleton(self, name: str):
        """
        Retrieve a single UTF-8 string from the UnionArray. The string is cloned.
        """
        ...

    def primitive_array(self, name: str):
        """
        Retrieve a primitive array from the UnionArray. The array is converted to a NumPy array with zero-copy.
        """
        ...

    def utf8_array(self, name: str):
        """
        Retrieve a UTF-8 array from the UnionArray. The array is converted to a Python list and so is cloned.
        """
        ...
