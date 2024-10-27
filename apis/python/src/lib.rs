use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};

pub mod datatypes;

#[pymodule]
fn fastformat(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(datatypes::datatypes))?;

    m.setattr("__version__", env!("CARGO_PKG_VERSION"))?;
    m.setattr("__author__", "Dora-rs Authors")?;

    Ok(())
}
