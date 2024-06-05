use pyo3::prelude::*;
use pyo3::types::PyList;

#[derive(Debug)]
pub struct Slider {
    pub min: f64,
    pub init: f64,
    pub max: f64,
}

pub fn parse_sliders(py_sliders: &Bound<'_, PyList>) -> PyResult<Vec<Slider>> {
    let mut sliders = Vec::new();

    for py_slider in py_sliders {
        let min: f64 = py_slider.getattr("min")?.extract()?;
        let init: f64 = py_slider.getattr("init")?.extract()?;
        let max: f64 = py_slider.getattr("max")?.extract()?;
        sliders.push(Slider { min, init, max });
    }

    Ok(sliders)
}
