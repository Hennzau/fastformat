use pyo3::prelude::*;

pub mod bbox;
pub mod image;

#[pymodule]
pub fn datatypes(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<image::Image>()?;
    m.add_class::<bbox::BBox>()?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
}
