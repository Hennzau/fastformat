from typing import Self
from typing_extensions import List

import numpy as np
import pyarrow as pa

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
