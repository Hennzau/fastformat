from ..fastformat import datatypes

Image = datatypes.Image

def _image_from_arrow(array_data):
    from fastformat.converter.arrow import ArrowViewer

    viewer = ArrowViewer(array_data)
    encoding = viewer.utf8_singleton('encoding')

    constructor = Image.new_rgb8 if encoding == 'RGB8' else Image.new_bgr8 if encoding == 'BGR8' else Image.new_gray8 if encoding == 'GRAY8' else None

    if constructor is None:
        raise ValueError(f"Unsupported encoding")

    return constructor(
        width=viewer.primitive_singleton('width'),
        height=viewer.primitive_singleton('height'),
        data=viewer.primitive_array('data'),
        name=viewer.utf8_singleton('name'),
    )

Image.from_arrow = _image_from_arrow

BBox = datatypes.BBox

def _bbox_from_arrow(array_data):
    from fastformat.converter.arrow import ArrowViewer

    viewer = ArrowViewer(array_data)
    encoding = viewer.utf8_singleton('encoding')

    constructor = BBox.new_xyxy if encoding == 'XYXY' else BBox.new_xywh if encoding == 'XYWH' else None

    if constructor is None:
        raise ValueError(f"Unsupported encoding")

    return constructor(
        data=viewer.primitive_array('data'),
        confidence=viewer.primitive_array('confidence'),
        label=viewer.utf8_array('label'),
    )

BBox.from_arrow = _bbox_from_arrow
