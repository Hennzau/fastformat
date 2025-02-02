use std::collections::HashMap;

use arrow::array::StringArray;
use eyre::OptionExt;

pub struct ArrowDataConsumer {
    array_data: HashMap<String, arrow::array::ArrayData>,
}

impl ArrowDataConsumer {
    pub fn new(array_data: arrow::array::ArrayData) -> eyre::Result<Self> {
        use arrow::array::Array;

        let array = arrow::array::UnionArray::from(array_data);

        let mut result = HashMap::new();

        let (union_fields, _, _, children) = array.into_parts();

        for (a, b) in union_fields.iter() {
            let child = children
                .get(a as usize)
                .ok_or_eyre(eyre::eyre!(
                    format!(
                        "Invalid union array field {}'s index (= {}). Must be >= 0 and correspond to children index in the array",
                        b, a
                    ),
                ))?
                .clone()
                .into_data();

            result.insert(b.name().to_string(), child);
        }

        Ok(Self { array_data: result })
    }

    pub fn optional_primitive_singleton<T: arrow::datatypes::ArrowPrimitiveType>(
        &mut self,
        field: &str,
    ) -> eyre::Result<Option<T::Native>> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let array = arrow::array::PrimitiveArray::<T>::from(data);
        let (_, buffer, null_buffer) = array.into_parts();

        if null_buffer.is_some() {
            return Ok(None);
        }

        let inner = buffer.into_inner();

        let slice = inner.typed_data::<T::Native>();

        Ok(Some(slice.first().cloned().ok_or_eyre(eyre::eyre!(
            format!(
                "Failed to get the first element of the buffer for field {}",
                field
            )
        ))?))
    }

    pub fn primitive_singleton<T: arrow::datatypes::ArrowPrimitiveType>(
        &mut self,
        field: &str,
    ) -> eyre::Result<T::Native> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let array = arrow::array::PrimitiveArray::<T>::from(data);
        let (_, buffer, _) = array.into_parts();

        let inner = buffer.into_inner();

        let slice = inner.typed_data::<T::Native>();

        slice.first().cloned().ok_or_eyre(eyre::eyre!(format!(
            "Failed to get the first element of the buffer for field {}",
            field
        )))
    }

    pub fn optional_utf8_singleton(&mut self, field: &str) -> eyre::Result<Option<String>> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let array = arrow::array::StringArray::from(data);
        let (offset_buffer, buffer, nullbuffer) = array.into_parts();

        if nullbuffer.is_some() {
            return Ok(None);
        }

        let slice = buffer.as_slice();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let last_offset = iterator.next().cloned().ok_or_eyre(eyre::eyre!(format!(
            "No offset associated with field {}",
            field
        )))? as usize;

        let slice = &slice[0..last_offset];

        Ok(Some(
            String::from_utf8(slice.to_vec()).map_err(|e| eyre::eyre!(e))?,
        ))
    }

    pub fn utf8_singleton(&mut self, field: &str) -> eyre::Result<String> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let array = arrow::array::StringArray::from(data);
        let (offset_buffer, buffer, _) = array.into_parts();

        let slice = buffer.as_slice();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let last_offset = iterator.next().cloned().ok_or_eyre(eyre::eyre!(format!(
            "No offset associated with field {}",
            field
        )))? as usize;

        let slice = &slice[0..last_offset];

        String::from_utf8(slice.to_vec()).map_err(|e| eyre::eyre!(e))
    }

    pub fn primitive_vec<T: arrow::datatypes::ArrowPrimitiveType>(
        &mut self,
        field: &str,
    ) -> eyre::Result<Vec<T::Native>> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let array = arrow::array::PrimitiveArray::<T>::from(data);
        let (_, buffer, _) = array.into_parts();

        let inner = buffer.into_inner();

        inner
            .into_vec::<T::Native>()
            .map_err(|_| eyre::eyre!("Invalid primitive array type. Or the buffer is shared. If you're not sure that the buffer is owned, use primitive_array_view instead."))
    }

    pub fn primitive_arrow<T: arrow::datatypes::ArrowPrimitiveType>(
        &mut self,
        field: &str,
    ) -> eyre::Result<arrow::array::PrimitiveArray<T>> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        Ok(arrow::array::PrimitiveArray::<T>::from(data))
    }

    pub fn utf8_vec(&mut self, field: &str) -> eyre::Result<Vec<String>> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let array = arrow::array::StringArray::from(data);
        let (offset_buffer, buffer, _) = array.into_parts();

        let slice = buffer.as_slice();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let mut last_offset = 0;

        iterator
            .map(|&offset| {
                let offset = offset as usize;
                let slice = &slice[last_offset..offset];
                last_offset = offset;

                String::from_utf8(slice.to_vec()).map_err(|e| eyre::eyre!(e))
            })
            .collect::<eyre::Result<Vec<String>>>()
    }

    pub fn utf8_arrow(&mut self, field: &str) -> eyre::Result<StringArray> {
        let data = self
            .array_data
            .remove(field)
            .ok_or_eyre(eyre::eyre!(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        Ok(arrow::array::StringArray::from(data))
    }
}
