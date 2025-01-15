use super::{encoding::Encoding, BBox};

use fastformat_converter::arrow::{
    builder::ArrowDataBuilder, consumer::ArrowDataConsumer, FromArrow, IntoArrow,
};

impl IntoArrow for BBox {
    fn into_arrow(self) -> eyre::Result<arrow::array::ArrayData> {
        let builder = ArrowDataBuilder::default()
            .push_primitive_arrow("data", self.data)
            .push_primitive_arrow("confidence", self.confidence)
            .push_utf8_arrow("label", self.label)
            .push_utf8_singleton("encoding", self.encoding.to_string());

        builder.build()
    }
}
impl FromArrow for BBox {
    fn from_arrow(array_data: arrow::array::ArrayData) -> eyre::Result<Self>
    where
        Self: Sized,
    {
        let mut consumer = ArrowDataConsumer::new(array_data)?;

        let data = consumer.primitive_arrow::<arrow::datatypes::Float32Type>("data")?;
        let confidence = consumer.primitive_arrow::<arrow::datatypes::Float32Type>("confidence")?;
        let label = consumer.utf8_arrow("label")?;

        let encoding = Encoding::from_string(consumer.utf8_singleton("encoding")?)?;

        Ok(Self {
            data,
            confidence,
            label,
            encoding,
        })
    }
}

mod tests {

    #[test]
    fn test_arrow_zero_copy_conversion() {
        use crate::bbox::BBox;
        use fastformat_converter::arrow::{FromArrow, IntoArrow};

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let original_buffer_address = flat_bbox.as_ptr();

        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let bbox_buffer_address = xyxy_bbox.data.values().as_ptr();

        let arrow_bbox = xyxy_bbox.into_arrow().unwrap();

        let xyxy_bbox = BBox::from_arrow(arrow_bbox).unwrap();
        let xyxy_bbox_buffer = xyxy_bbox.data.values().as_ptr();

        let xywh_bbox = xyxy_bbox.into_xywh().unwrap();
        let xywh_bbox_buffer = xywh_bbox.data.values().as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_eq!(bbox_buffer_address, xyxy_bbox_buffer);
        assert_eq!(xyxy_bbox_buffer, xywh_bbox_buffer);
    }

    #[test]
    fn test_arrow_zero_copy_read_only() {
        use crate::bbox::BBox;
        use fastformat_converter::arrow::{FromArrow, IntoArrow};

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let original_buffer_address = flat_bbox.as_ptr();

        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let bbox_buffer_address = xyxy_bbox.data.values().as_ptr();

        let arrow_bbox = xyxy_bbox.into_arrow().unwrap();
        let _save = arrow_bbox.clone();

        let xyxy_bbox = BBox::from_arrow(arrow_bbox).unwrap();
        let xyxy_bbox_buffer = xyxy_bbox.data.values().as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_eq!(bbox_buffer_address, xyxy_bbox_buffer);
    }

    #[test]
    fn test_arrow_zero_copy_copy_on_write() {
        use crate::bbox::BBox;
        use fastformat_converter::arrow::{FromArrow, IntoArrow};

        let flat_bbox = vec![1.0, 1.0, 2.0, 2.0];
        let original_buffer_address = flat_bbox.as_ptr();

        let confidence = vec![0.98];
        let label = vec!["cat".to_string()];

        let xyxy_bbox = BBox::new_xyxy(flat_bbox, confidence, label).unwrap();
        let bbox_buffer_address = xyxy_bbox.data.values().as_ptr();

        let arrow_bbox = xyxy_bbox.into_arrow().unwrap();
        let _save = arrow_bbox.clone();

        let xyxy_bbox = BBox::from_arrow(arrow_bbox).unwrap();
        let xywh_bbox = xyxy_bbox.into_xywh().unwrap();

        let final_bbox_buffer = xywh_bbox.data.values().as_ptr();

        assert_eq!(original_buffer_address, bbox_buffer_address);
        assert_ne!(bbox_buffer_address, final_bbox_buffer);
    }
}
