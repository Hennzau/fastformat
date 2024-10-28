use arrow::array::{ArrayData, UInt16Array, UInt8Array};

#[derive(Debug)]
pub enum ImageData {
    U8(UInt8Array),
    U16(UInt16Array),
}

impl ImageData {
    pub fn len(&self) -> usize {
        match self {
            Self::U8(data) => data.len(),
            Self::U16(data) => data.len(),
        }
    }

    pub fn as_ptr(&self) -> *const u64 {
        match self {
            Self::U8(data) => data.values().as_ptr() as *const u64,
            Self::U16(data) => data.values().as_ptr() as *const u64,
        }
    }

    pub fn into_vec_u8(self) -> eyre::Result<Vec<u8>> {
        match self {
            Self::U8(data) => {
                let (_, data, _) = data.into_parts();

                let buffer = data.into_inner();

                match buffer.into_vec::<u8>() {
                    Ok(vec) => Ok(vec),
                    Err(buffer) => Ok(buffer.typed_data::<u8>().to_vec()),
                }
            }
            _ => Err(eyre::Report::msg("Can't convert data to u8")),
        }
    }

    pub fn into_arrow_u8(self) -> eyre::Result<UInt8Array> {
        match self {
            Self::U8(data) => Ok(data),
            _ => Err(eyre::Report::msg("Can't convert data to u8")),
        }
    }

    pub fn as_u8(&self) -> eyre::Result<&[u8]> {
        match self {
            Self::U8(data) => Ok(data.values().as_ref()),
            _ => Err(eyre::Report::msg("Can't convert data to u8")),
        }
    }

    pub fn from_vec_u8(data: Vec<u8>) -> Self {
        Self::U8(UInt8Array::from(data))
    }

    pub fn from_array_data_u8(data: ArrayData) -> Self {
        Self::U8(UInt8Array::from(data))
    }

    pub fn from_array_u8(data: UInt8Array) -> Self {
        Self::U8(data)
    }
}
