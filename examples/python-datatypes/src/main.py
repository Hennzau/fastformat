import pyarrow as pa
import numpy as np

from fastformat.datatypes import Image, BBox

bgr8_image = Image.new_bgr8(np.array([0, 0, 0], dtype=np.uint8), 1, 1, "test")

array_data = bgr8_image.into_arrow()

reconstructed_image = Image.from_arrow(array_data)
print(reconstructed_image.name())

xyxy_bbox = BBox.new_xyxy(np.array([0, 0, 1, 1], dtype=np.float32), np.array([0.5], dtype=np.float32), ["test"])

array_data = xyxy_bbox.into_arrow()

reconstructed_bbox = BBox.from_arrow(array_data)
