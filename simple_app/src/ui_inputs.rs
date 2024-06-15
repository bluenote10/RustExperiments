use cushy::figures::units::Px;
use cushy::value::{Destination, Dynamic, Source, Switchable};
use cushy::widget::MakeWidget;
use cushy::widget::WidgetList;
use cushy::widgets::slider::Slidable;
use cushy::widgets::Space;
use pyo3::prelude::*;
use pyo3::types::PyFunction;

use crate::conversion::{parse_callback_return, CallbackReturn, Input};
use crate::ui_plots::plots_widget;

pub fn input_widget(inputs: &[Input], py_callback: Py<PyFunction>) -> impl MakeWidget {
    let cb_return_dynamic: Dynamic<Option<CallbackReturn>> = Dynamic::new(None);

    let mut widget_list = WidgetList::new();
    for input in inputs.iter() {
        // TODO: Clean up the duplication here...
        if let Input::Slider(slider) = input {
            let py_callback = py_callback.clone();
            let py_slider = slider.py_slider.clone();
            let cb_return_dynamic = cb_return_dynamic.clone();

            // Temporary work-around for initial callback call.
            // https://github.com/khonsulabs/cushy/issues/156#issuecomment-2152677089
            let callback = move |value: &f64| {
                let result = Python::with_gil(|py| -> PyResult<()> {
                    py_slider.setattr(py, "value", *value)?;

                    let cb_return = py_callback.call_bound(py, (), None)?;
                    let cb_return = parse_callback_return(py, cb_return)?;

                    cb_return_dynamic.set(Some(cb_return));
                    Ok(())
                });
                if let Err(e) = result {
                    println!("Error on calling callback: {}", e);
                }
            };

            let value = Dynamic::new(slider.init);
            value.map_ref(&callback);
            value.for_each(callback).persist();

            let label_row = slider
                .name
                .clone()
                .small()
                .and(value.map_each(|x| format!("{}", x)).small())
                .into_columns();

            let slider = value.clone().slider_between(slider.min, slider.max);
            widget_list = widget_list.and(label_row.and(slider).into_rows().contain());
        } else if let Input::IntSlider(slider) = input {
            let py_callback = py_callback.clone();
            let py_slider = slider.py_slider.clone();
            let cb_return_dynamic = cb_return_dynamic.clone();

            // Temporary work-around for initial callback call.
            // https://github.com/khonsulabs/cushy/issues/156#issuecomment-2152677089
            let callback = move |value: &i64| {
                let result = Python::with_gil(|py| -> PyResult<()> {
                    py_slider.setattr(py, "value", *value)?;

                    let cb_return = py_callback.call_bound(py, (), None)?;
                    let cb_return = parse_callback_return(py, cb_return)?;

                    cb_return_dynamic.set(Some(cb_return));
                    Ok(())
                });
                if let Err(e) = result {
                    println!("Error on calling callback: {}", e);
                }
            };

            let value = Dynamic::new(slider.init);
            value.map_ref(&callback);
            value.for_each(callback).persist();

            let label_row = slider
                .name
                .clone()
                .small()
                .and(value.map_each(|x| format!("{}", x)).small())
                .into_columns();

            let slider = value.clone().slider_between(slider.min, slider.max);
            widget_list = widget_list.and(label_row.and(slider).into_rows().contain());
        }
    }

    let content = cb_return_dynamic.switcher(|cb_result, _active| {
        let Some(cb_result) = cb_result else {
            return Space::clear().make_widget();
        };
        match cb_result {
            CallbackReturn::Outputs(plots) => plots_widget(plots.clone()).make_widget(),
            CallbackReturn::Inputs(inputs, callback) => {
                input_widget(&inputs, callback.clone()).make_widget()
            }
        }
    });
    widget_list
        .into_rows()
        .contain()
        .width(Px::new(300))
        .and(content.expand())
        .into_columns()
}
