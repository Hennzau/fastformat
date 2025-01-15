use fastformat_rs::prelude::*;

#[derive(Debug)]
pub struct CustomDataTypeShared {
    size: u32,
    label: String,
    ranges: UInt8Array,
}

impl IntoArrow for CustomDataTypeShared {
    fn into_arrow(self) -> eyre::Result<ArrowArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_singleton::<UInt32Type>("size", self.size)
            .push_utf8_singleton("label", self.label)
            .push_primitive_arrow("ranges", self.ranges);

        builder.build()
    }
}

impl FromArrow for CustomDataTypeShared {
    fn from_arrow(array_data: ArrowArrayData) -> eyre::Result<Self> {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let size = consumer.primitive_singleton::<UInt32Type>("size")?;
        let label = consumer.utf8_singleton("label")?;
        let ranges = consumer.primitive_arrow::<UInt8Type>("ranges")?;

        Ok(Self {
            size,
            label,
            ranges,
        })
    }
}

fn main() -> eyre::Result<()> {
    let custom_data = CustomDataTypeShared {
        size: 42,
        label: "Hello, World!".to_string(),
        ranges: UInt8Array::from(vec![1, 2, 3, 4, 5]),
    };
    let ptr1 = custom_data.ranges.values().as_ptr();

    // Consume the custom data and convert it into Arrow data
    let arrow_data = custom_data.into_arrow()?;
    let _save = arrow_data.clone(); // Force shared ownership

    let custom_data = CustomDataTypeShared::from_arrow(arrow_data)?;
    let ptr2 = custom_data.ranges.values().as_ptr();

    assert_eq!(ptr1, ptr2);

    Ok(())
}
