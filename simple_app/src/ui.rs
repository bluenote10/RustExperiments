use cushy::widget::MakeWidget;
use cushy::Run;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyFunction;

use crate::conversion::Input;
use crate::ui_inputs::input_widget;

pub fn run_ui(sliders: &[Input], callback: &Bound<'_, PyFunction>) -> PyResult<()> {
    let py = callback.py();
    let callback = callback.clone().unbind();

    py.allow_threads(|| {
        let window = input_widget(sliders, callback)
            .into_window()
            .titled("pico app");
        let result = window.run();
        result.map_err(|e| PyRuntimeError::new_err(format!("Failed to run widget: {}", e)))
    })
}
