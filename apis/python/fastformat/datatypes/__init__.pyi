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

class BBox:
    @staticmethod
    def new_xyxy(data: np.ndarray, confidence: np.ndarray, label: List[str]) -> BBox:
        ...

    @staticmethod
    def new_xywh(data: np.ndarray, confidence: np.ndarray, label: List[str]) -> BBox:
        ...

    def into_xyxy(self) -> Self:
        ...

    def into_xywh(self) -> Self:
        ...

    def into_arrow(self) -> pa.UnionArray:
        ...

    @staticmethod
    def from_arrow(array_data: pa.UnionArray) -> BBox:
        ...
