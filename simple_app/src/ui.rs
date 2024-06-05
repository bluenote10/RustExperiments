use cushy::value::{Dynamic, Source};
use cushy::widget::{MakeWidget, WidgetList};
use cushy::widgets::slider::Slidable;
use cushy::widgets::Canvas;
use cushy::Run;
use plotters::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyFunction;

use super::conversion::Slider;

// This is copied from the sierpinski.rs example in the plotters repository.
// This just demonstrates that any `plotters` code that renders to a
// `DrawingArea` can be used with a `Canvas`.
pub fn sierpinski_carpet<A>(
    depth: u32,
    drawing_area: &DrawingArea<A, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>>
where
    A: DrawingBackend,
    A::ErrorType: 'static,
{
    if depth > 0 {
        let sub_areas = drawing_area.split_evenly((3, 3));
        for (idx, sub_area) in (0..).zip(sub_areas.iter()) {
            if idx != 4 {
                sub_area.fill(&BLUE)?;
                sierpinski_carpet(depth - 1, sub_area)?;
            } else {
                sub_area.fill(&WHITE)?;
            }
        }
    }
    Ok(())
}

fn ui_widget(sliders: &[Slider], callback: Py<PyFunction>) -> impl MakeWidget {
    let depth = Dynamic::new(1).with_for_each({
        let callback = callback.clone();
        move |value| {
            println!("value: {value}");
            Python::with_gil(|py| {
                let result = callback.call_bound(py, (*value,), None);
                if let Err(e) = result {
                    println!("Error on calling callback: {}", e);
                }
            });
        }
    });

    let mut widget_list = WidgetList::new();
    for slider in sliders.iter() {
        let callback = callback.clone();
        let py_slider = slider.py_slider.clone();
        let value = Dynamic::new(slider.init).with_for_each(move |value| {
            println!("value: {value}");
            let result = Python::with_gil(|py| -> PyResult<()> {
                py_slider.setattr(py, "value", *value)?;
                callback.call_bound(py, (), None)?;
                Ok(())
            });
            if let Err(e) = result {
                println!("Error on calling callback: {}", e);
            }
        });

        let slider = value.clone().slider_between(slider.min, slider.max);

        widget_list = widget_list.and(slider);
        //widget_list = widget_list.and::<String>(format!("{:?}", slider));
    }

    // Temporary work-around for initial callback call.
    Python::with_gil(|py| {
        callback.call_bound(py, (), None).unwrap();
    });

    "Depth"
        .and(depth.clone().slider_between(1, 5))
        .and(
            Canvas::new({
                move |context| {
                    let depth = depth.get_tracking_redraw(context);
                    sierpinski_carpet(depth, &context.gfx.as_plot_area()).unwrap();
                }
            })
            .expand(),
        )
        .and(widget_list.into_rows())
        .into_rows()
}

pub fn run_ui(sliders: &[Slider], callback: &Bound<'_, PyFunction>) {
    let py = callback.py();
    let callback = callback.clone().unbind();

    py.allow_threads(|| {
        let result = ui_widget(sliders, callback).run();
        // TODO: Can this be turned into a Python exception?
        if let Err(e) = result {
            println!("Failed to run widget: {}", e);
        }
    });
}
