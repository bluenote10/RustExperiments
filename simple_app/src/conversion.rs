use std::ops::Range;

use pyo3::prelude::*;
use pyo3::types::PyList;

#[derive(Debug)]
pub struct Slider {
    pub name: String,
    pub min: f64,
    pub init: f64,
    pub max: f64,
    pub py_slider: PyObject,
}

pub fn parse_sliders(py_sliders: &Bound<'_, PyList>) -> PyResult<Vec<Slider>> {
    let mut sliders = Vec::new();

    for py_slider in py_sliders {
        let name: String = py_slider.getattr("name")?.extract()?;
        let min: f64 = py_slider.getattr("min")?.extract()?;
        let init: f64 = py_slider.getattr("value")?.extract()?;
        let max: f64 = py_slider.getattr("max")?.extract()?;
        sliders.push(Slider {
            name,
            min,
            init,
            max,
            py_slider: py_slider.clone().unbind(),
        });
    }

    Ok(sliders)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plot {
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub x_limits: Range<f32>,
    pub y_limits: Range<f32>,
}

pub fn parse_callback_return(py: Python<'_>, cb_return: PyObject) -> PyResult<Option<Vec<Plot>>> {
    let cb_return = cb_return.bind(py);
    /*    if cb_return.hasattr("__name__")?
           && cb_return
               .getattr("__name__")?
               .eq::<PyString>("Outputs".try_into()?)?
    */
    if cb_return.get_type().name()? == "Outputs" {
        return Ok(Some(parse_outputs(
            py,
            cb_return.getattr("outputs")?.into(),
        )?));
    }
    Ok(None)
}

pub fn parse_outputs(py: Python<'_>, outputs: PyObject) -> PyResult<Vec<Plot>> {
    let output = outputs.bind(py);
    let mut results = Vec::new();
    // TODO: This can be improved a lot. Most likely we could leverage the buffer
    // protocol (or https://github.com/PyO3/rust-numpy) to make this zero copy?
    for object in output.iter()? {
        let object = object?;
        if object.hasattr("xs")?
            && object.hasattr("ys")?
            && object.hasattr("x_limits")?
            && object.hasattr("y_limits")?
        {
            let xs: Vec<f64> = object.getattr("xs")?.extract()?;
            let ys: Vec<f64> = object.getattr("ys")?.extract()?;
            let x_limits: (f64, f64) = object.getattr("x_limits")?.extract()?;
            let y_limits: (f64, f64) = object.getattr("y_limits")?.extract()?;
            results.push(Plot {
                xs,
                ys,
                x_limits: x_limits.0 as f32..x_limits.1 as f32,
                y_limits: y_limits.0 as f32..y_limits.1 as f32,
            });
        }
    }
    Ok(results)
}
