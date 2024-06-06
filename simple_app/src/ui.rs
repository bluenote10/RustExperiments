use cushy::value::{Destination, Dynamic, Source};
use cushy::widget::MakeWidget;
use cushy::widget::WidgetList;
use cushy::widgets::slider::Slidable;
use cushy::widgets::Canvas;
use cushy::Run;
use plotters::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyFunction;

use crate::conversion::{parse_output, Plot, Slider};

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

fn ui_widget(sliders: &[Slider], callback: Py<PyFunction>) -> impl MakeWidget {
    // TODO: Is there a better / more performant way to implement that? Apparently
    // we need `PartialEq` for `set` and `Clone` for `get_tracking_redraw`, which
    // both sound not particular efficient.
    let plots = Dynamic::new(Vec::<Plot>::new());

    let mut widget_list = WidgetList::new();
    for slider in sliders.iter() {
        let callback = callback.clone();
        let py_slider = slider.py_slider.clone();
        let plots = plots.clone();
        let value = Dynamic::new(slider.init).with_for_each(move |value| {
            let result = Python::with_gil(|py| -> PyResult<()> {
                py_slider.setattr(py, "value", *value)?;
                let new_plots = callback
                    .call_bound(py, (), None)
                    .and_then(|output| parse_output(py, output))?;
                plots.set(new_plots);
                Ok(())
            });
            if let Err(e) = result {
                println!("Error on calling callback: {}", e);
            }
        });

        let label_row = slider
            .name
            .clone()
            .small()
            .and(value.map_each(|x| format!("{}", x)).small())
            .into_columns();

        let slider = value.clone().slider_between(slider.min, slider.max);
        widget_list = widget_list.and(label_row.and(slider).into_rows().contain());
    }

    // Temporary work-around for initial callback call.
    let _ = Python::with_gil(|py| -> PyResult<()> {
        let new_plots = callback
            .call_bound(py, (), None)
            .and_then(|output| parse_output(py, output))?;
        plots.set(new_plots);
        Ok(())
    });

    Canvas::new({
        {
            let plots = plots.clone();
            move |context| {
                let plots = plots.get_tracking_redraw(context);
                // TODO: Support more plots...
                if plots.len() > 0 {
                    render_plot(&plots[0], &context.gfx.as_plot_area()).unwrap();
                }
            }
        }
    })
    .expand()
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
