use arrow::{array::ArrayData, pyarrow::PyArrowType};
use pyo3::prelude::*;

#[pyclass]
pub struct BBox(Option<fastformat_rs::prelude::BBox<'static>>);

#[pymethods]
impl BBox {
    #[staticmethod]
    pub fn new_xywh(data: Vec<f32>, confidence: Vec<f32>, label: Vec<String>) -> PyResult<Self> {
        let bbox = fastformat_rs::prelude::BBox::new_xywh(data, confidence, label)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(bbox)))
    }

    #[staticmethod]
    pub fn new_xyxy(data: Vec<f32>, confidence: Vec<f32>, label: Vec<String>) -> PyResult<Self> {
        let bbox = fastformat_rs::prelude::BBox::new_xyxy(data, confidence, label)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(bbox)))
    }

    pub fn into_xywh(&mut self) -> PyResult<Self> {
        let bbox = self
            .0
            .take()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("BBox is None"))?
            .into_xywh()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(bbox)))
    }

    pub fn into_xyxy(&mut self) -> PyResult<Self> {
        let bbox = self
            .0
            .take()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("BBox is None"))?
            .into_xyxy()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(Self(Some(bbox)))
    }

    pub fn into_arrow(&mut self) -> PyResult<PyArrowType<ArrayData>> {
        use fastformat_rs::prelude::IntoArrow;

        let bbox = self
            .0
            .take()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("BBox is None"))?
            .into_arrow()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        Ok(PyArrowType::from(bbox))
    }
}
