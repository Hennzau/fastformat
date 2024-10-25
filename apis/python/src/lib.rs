use pyo3::{prelude::*, wrap_pyfunction};

pub mod datatypes;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("hello fastformat".to_string())
}

#[pymodule]
fn pyfastformat(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, &m)?)?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
}
