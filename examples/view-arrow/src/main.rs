use std::borrow::Cow;

use fastformat::prelude::*;

#[derive(Debug)]
pub struct CustomDataTypeView<'a> {
    size: u32,
    label: String,
    ranges: Cow<'a, [u8]>,
}

impl IntoArrow for CustomDataTypeView<'_> {
    fn into_arrow(self) -> eyre::Result<ArrowArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_singleton::<UInt32Type>("size", self.size)
            .push_utf8_singleton("label", self.label)
            .push_primitive_array::<UInt8Type>("ranges", self.ranges.into_owned());

        builder.build()
    }

    fn from_arrow(array_data: ArrowArrayData) -> eyre::Result<Self> {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let size = consumer.primitive_singleton::<UInt32Type>("size")?;
        let label = consumer.utf8_singleton("label")?;
        let ranges = consumer.primitive_array::<UInt8Type>("ranges")?;

        Ok(Self {
            size,
            label,
            ranges: Cow::Owned(ranges),
        })
    }
}

impl<'a> ViewArrow<'a> for CustomDataTypeView<'a> {
    fn viewer(array_data: ArrowArrayData) -> eyre::Result<ArrowDataViewer> {
        ArrowDataViewer::new(array_data)?.load_primitive::<UInt8Type>("ranges")
    }
    fn view_arrow(viewer: &'a ArrowDataViewer) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let size = viewer.primitive_singleton::<UInt32Type>("size")?;
        let label = viewer.utf8_singleton("label")?;
        let ranges = viewer.primitive_array::<UInt8Type>("ranges")?;

        Ok(Self {
            size,
            label,
            ranges: Cow::Borrowed(ranges),
        })
    }
}

fn main() -> eyre::Result<()> {
    let custom_data = CustomDataTypeView {
        size: 42,
        label: "Hello, World!".to_string(),
        ranges: Cow::Owned(vec![1, 2, 3, 4, 5]),
    };
    let ptr1 = custom_data.ranges.as_ptr();

    // Consume the custom data and convert it into Arrow data
    let arrow_data = custom_data.into_arrow()?;

    // View the Arrow data
    let viewer = CustomDataTypeView::viewer(arrow_data)?;

    let custom_data = CustomDataTypeView::view_arrow(&viewer)?;
    let ptr2 = custom_data.ranges.as_ptr();

    assert_eq!(ptr1, ptr2);

    Ok(())
}
