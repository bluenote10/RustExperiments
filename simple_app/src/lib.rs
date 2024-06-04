mod plotters;

use pyo3::prelude::*;
use pyo3::types::{PyFunction, PyList};

use plotters::run_plotters;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn run(names: &Bound<'_, PyList>, callback: &Bound<'_, PyFunction>) -> PyResult<()> {
    callback.call((1, 2), None)?;
    run_plotters(names);
    Ok(())
}

#[pymodule]
fn _simple_app(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
