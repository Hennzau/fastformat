use arrow::{array::ArrayData, pyarrow::PyArrowType};
use fastformat_rs::prelude::FromArrow;
use pyo3::prelude::*;

#[pyclass]
pub struct Image(Option<fastformat_rs::prelude::Image>);

#[pymethods]
impl Image {
    #[staticmethod]
    pub fn new_bgr8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> PyResult<Self> {
        let image = fastformat_rs::prelude::Image::new_bgr8(data, width, height, name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(image)))
    }

    #[staticmethod]
    pub fn new_rgb8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> PyResult<Self> {
        let image = fastformat_rs::prelude::Image::new_rgb8(data, width, height, name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(image)))
    }

    #[staticmethod]
    pub fn new_gray8(data: Vec<u8>, width: u32, height: u32, name: Option<&str>) -> PyResult<Self> {
        let image = fastformat_rs::prelude::Image::new_gray8(data, width, height, name)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(image)))
    }

    pub fn into_rgb8(&mut self) -> PyResult<Self> {
        let image = self
            .0
            .take()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Image is None"))?
            .into_rgb8()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(image)))
    }

    pub fn into_bgr8(&mut self) -> PyResult<Self> {
        let image = self
            .0
            .take()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Image is None"))?
            .into_bgr8()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(image)))
    }

    pub fn into_arrow(&mut self) -> PyResult<PyArrowType<ArrayData>> {
        use fastformat_rs::prelude::IntoArrow;

        let image = self
            .0
            .take()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Image is None"))?
            .into_arrow()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(PyArrowType::from(image))
    }

    #[staticmethod]
    pub fn from_arrow(array_data: PyArrowType<ArrayData>) -> PyResult<Self> {
        let image = fastformat_rs::prelude::Image::from_arrow(array_data.0)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(image)))
    }

    pub fn width(&self) -> PyResult<u32> {
        Ok(self.0.as_ref().map(|image| image.width).unwrap_or(0))
    }

    pub fn height(&self) -> PyResult<u32> {
        Ok(self.0.as_ref().map(|image| image.height).unwrap_or(0))
    }

    pub fn name(&self) -> PyResult<Option<String>> {
        Ok(self
            .0
            .as_ref()
            .map(|image| image.name.clone())
            .unwrap_or(None))
    }
}
