use cushy::widget::Widget;
use cushy::widgets::Canvas;
use plotters::prelude::*;

use crate::conversion::Plot;

pub fn plots_widget(plots: Vec<Plot>) -> impl Widget {
    // TODO: Support more plots, either by using multiple canvases, or plotting
    // them all into one?
    Canvas::new({
        move |context| {
            if plots.len() > 0 {
                render_plot(&plots[0], &context.gfx.as_plot_area()).unwrap();
            }
        }
    })
}

fn render_plot<A>(
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
