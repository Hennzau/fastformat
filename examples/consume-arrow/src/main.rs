use fastformat::prelude::*;

#[derive(Debug)]
pub struct CustomDataType {
    size: u32,
    label: String,
    ranges: Vec<u8>,
}

impl IntoArrow for CustomDataType {
    fn into_arrow(self) -> eyre::Result<ArrowArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_singleton::<UInt32Type>("size", self.size)
            .push_utf8_singleton("label", self.label)
            .push_primitive_array::<UInt8Type>("ranges", self.ranges);

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
            ranges,
        })
    }
}

fn main() -> eyre::Result<()> {
    let custom_data = CustomDataType {
        size: 42,
        label: "Hello, World!".to_string(),
        ranges: vec![1, 2, 3, 4, 5],
    };

    let ptr1 = custom_data.ranges.as_ptr();

    // Consume the custom data and convert it into Arrow data
    let arrow_data = custom_data.into_arrow()?;

    // Convert the Arrow data back into custom data
    let custom_data = CustomDataType::from_arrow(arrow_data)?;

    let ptr2 = custom_data.ranges.as_ptr();

    // The pointers should be the same as the data was not copied
    assert_eq!(ptr1, ptr2);

    Ok(())
}
