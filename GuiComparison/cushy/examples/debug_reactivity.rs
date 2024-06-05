use cushy::figures::units::Lp;
use cushy::value::{Dynamic, IntoReader, Source};
use cushy::widget::MakeWidget;
use cushy::widgets::slider::Slidable;
use cushy::Run;

fn main() -> cushy::Result {
    let value = Dynamic::new(50_i32);

    // This doesn't run on the initial value, but subsequent values.
    let value = value.with_for_each(|value| {
        println!("value: {value} [with_for_each]");
    });

    // This doesn't run at all, probably due to dropped handle.
    let _ = value.for_each(|value| {
        println!("value: {value} [for_each, dropped handle]");
    });

    // Interestingly, a "dropped" map runs on the initial value, but not
    // on subsequent values.
    value.map_each(|value| {
        println!("value: {value} [map_each, unused]");
    });

    // This runs on all (initial and subsequent) values.
    let dummy = value.map_each(|value| {
        println!("value: {value} [map_each, used]");
        *value
    });

    "value: "
        .and(value.clone().into_label())
        .and(dummy.clone().into_label())
        .into_columns()
        .centered()
        .and(
            value
                .slider_between(0, 100)
                .width(Lp::points(400)..Lp::points(800)),
        )
        .into_rows()
        .contain()
        .centered()
        .run()
}
