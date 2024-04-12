use cushy::figures::units::Lp;
use cushy::value::{Dynamic, IntoReader, Source};
use cushy::widget::MakeWidget;
use cushy::widgets::checkbox::Checkable;
use cushy::widgets::slider::Slidable;
use cushy::Run;

fn main() -> cushy::Result {
    let value = Dynamic::new(50_i32);
    let frozen = Dynamic::new(false);

    "value: "
        .and(value.clone().into_label())
        .into_columns()
        .centered()
        .and(
            value
                .slider_between(0, 100)
                .width(Lp::points(400)..Lp::points(800)),
        )
        .into_rows()
        .with_enabled(frozen.map_each(|b| !b))
        .contain()
        .and(frozen.into_checkbox("Frozen"))
        .into_rows()
        .contain()
        .pad()
        .centered()
        .run()
}
