use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};

pub mod datatypes;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok("hello fastformat".to_string())
}

#[pymodule]
fn fastformat(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, &m)?)?;

    m.add_wrapped(wrap_pymodule!(datatypes::datatypes))?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
}
