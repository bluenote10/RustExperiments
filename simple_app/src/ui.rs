use cushy::figures::units::Px;
use cushy::value::{Destination, Dynamic, Source, Switchable};
use cushy::widget::MakeWidget;
use cushy::widget::{Widget, WidgetList};
use cushy::widgets::slider::Slidable;
use cushy::widgets::{Canvas, Space};
use cushy::Run;
use plotters::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyFunction;

use crate::conversion::{parse_callback_return, CallbackReturn, Plot, Slider};

pub fn render_plot<A>(
    plot: &Plot,
    root: &DrawingArea<A, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>>
where
    A: DrawingBackend,
    A::ErrorType: 'static,
{
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(plot.x_limits.clone(), plot.y_limits.clone())?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        plot.xs
            .iter()
            .zip(plot.ys.iter())
            .map(|(&x, &y)| (x as f32, y as f32)),
        &RED,
    ))?;

    Ok(())
}

fn ui_widget(inputs: &[Slider], py_callback: Py<PyFunction>) -> impl MakeWidget {
    let cb_return_dynamic: Dynamic<Option<CallbackReturn>> = Dynamic::new(None);

    let mut widget_list = WidgetList::new();
    for slider in inputs.iter() {
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
    }

    let content = cb_return_dynamic.switcher(|cb_result, _active| {
        let Some(cb_result) = cb_result else {
            return Space::clear().make_widget();
        };
        match cb_result {
            CallbackReturn::Outputs(plots) => plots_widget(plots.clone()).make_widget(),
            CallbackReturn::Inputs(inputs, callback) => {
                ui_widget(&inputs, callback.clone()).make_widget()
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

fn plots_widget(plots: Vec<Plot>) -> impl Widget {
    Canvas::new({
        move |context| {
            // TODO: Support more plots...
            if plots.len() > 0 {
                render_plot(&plots[0], &context.gfx.as_plot_area()).unwrap();
            }
        }
    })
}

/*
fn plots_widget(plots: Dynamic<Vec<Plot>>) -> impl Widget {
    Canvas::new({
        let plots = plots.clone();
        move |context| {
            let plots = plots.get_tracking_redraw(context);
            // TODO: Support more plots...
            if plots.len() > 0 {
                render_plot(&plots[0], &context.gfx.as_plot_area()).unwrap();
            }
        }
    })
}
*/

pub fn run_ui(sliders: &[Slider], callback: &Bound<'_, PyFunction>) -> PyResult<()> {
    let py = callback.py();
    let callback = callback.clone().unbind();

    py.allow_threads(|| {
        let result = ui_widget(sliders, callback).run();
        result.map_err(|e| PyRuntimeError::new_err(format!("Failed to run widget: {}", e)))
    })
}
