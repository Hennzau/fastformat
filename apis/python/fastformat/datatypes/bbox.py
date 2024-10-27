from ..fastformat import datatypes

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
