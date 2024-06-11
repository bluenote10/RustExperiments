use pyo3::prelude::*;
use pyo3::types::{PyFunction, PyList};

use super::conversion::parse_sliders;
use super::ui::run_ui;

#[pyfunction]
fn run(sliders: &Bound<'_, PyList>, callback: &Bound<'_, PyFunction>) -> PyResult<()> {
    let sliders = parse_sliders(sliders)?;
    run_ui(&sliders, callback)?;
    Ok(())
}

#[pymodule]
fn _simple_app(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
