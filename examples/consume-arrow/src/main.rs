use fastformat::prelude::*;

#[derive(Debug)]
pub struct CustomDataType {
    size: Option<u32>,
    label: Option<String>,
    ranges: Vec<u8>,
}

impl IntoArrow for CustomDataType {
    fn into_arrow(self) -> eyre::Result<ArrowArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_optional_primitive_singleton::<UInt32Type>("size", self.size)
            .push_optional_utf8_singleton("label", self.label)
            .push_primitive_array::<UInt8Type>("ranges", self.ranges);

        builder.build()
    }
}
impl FromArrow for CustomDataType {
    fn from_arrow(array_data: ArrowArrayData) -> eyre::Result<Self> {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let size = consumer.optional_primitive_singleton::<UInt32Type>("size")?;
        let label = consumer.optional_utf8_singleton("label")?;
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
        size: Some(42),
        label: Some("Hello, World!".to_string()),
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
    println!("{:?}", custom_data);

    Ok(())
}
