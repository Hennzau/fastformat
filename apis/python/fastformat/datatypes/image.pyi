from typing import Self
from typing_extensions import List

import numpy as np
import pyarrow as pa

class Image:

    @staticmethod
    def new_bgr8(data: np.ndarray, width: int, height: int, name: str) -> Image:
        ...

    @staticmethod
    def new_rgb8(data: np.ndarray, width: int, height: int, name: str) -> Image:
        ...

    @staticmethod
    def new_gray8(data: np.ndarray, width: int, height: int, name: str) -> Image:
        ...

    def into_rgb8(self) -> Self:
        ...

    def into_bgr8(self) -> Self:
        ...

    def into_arrow(self) -> pa.UnionArray:
        ...

    @staticmethod
    def from_arrow(array_data: pa.UnionArray) -> Image:
        ...

    def width(self) -> int:
        ...

    def height(self) -> int:
        ...

    def name(self) -> str:
        ...

    def encoding(self) -> str:
        ...
