use cushy::figures::units::Px;
use cushy::widget::MakeWidget;
use cushy::widgets::Canvas;
use cushy::Run;
use plotters::prelude::*;

pub fn basic_plot<A>(
    root: &DrawingArea<A, plotters::coord::Shift>,
) -> Result<(), Box<dyn std::error::Error>>
where
    A: DrawingBackend,
    A::ErrorType: 'static,
{
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-2f32..2f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-100..=100).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &RED,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn sidebar() -> impl MakeWidget {
    "Dummy A"
        .into_button()
        .and("Dummy B".into_button())
        .and("Dummy C".into_button())
        .into_rows()
        .contain()
}

fn content() -> impl MakeWidget {
    Canvas::new({
        move |context| {
            basic_plot(&context.gfx.as_plot_area()).unwrap();
        }
    })
    .width(Px::new(200)..)
    .height(Px::new(200)..)
    .expand()
}

pub fn main() -> cushy::Result<()> {
    sidebar().and(content()).into_columns().expand().run()
}
