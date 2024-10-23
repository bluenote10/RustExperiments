use cushy::animation::ZeroToOne;
use cushy::figures::units::Px;
use cushy::figures::Zero;
use cushy::value::Dynamic;
use cushy::widget::MakeWidget;
use cushy::widgets::progress::Progressable;
use cushy::window::ThemeMode;
use cushy::Run;

fn content() -> impl MakeWidget {
    let progress = Dynamic::new(ZeroToOne::ZERO);
    progress
        .clone()
        .progress_bar()
        .width(Px::new(100)..) // (1) using just `Px::new(100)` works
        .contain()
        .centered()
        .expand()
}

pub fn main() -> cushy::Result<()> {
    content()
        .themed_mode(ThemeMode::Dark) // (2) seems to have no effect
        .run()
}
