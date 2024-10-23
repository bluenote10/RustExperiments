use std::thread;
use std::time::Duration;

use cushy::animation::ZeroToOne;
use cushy::figures::units::Px;
use cushy::value::{Destination, Dynamic};
use cushy::widget::MakeWidget;
use cushy::widgets::progress::{Progress, Progressable};
use cushy::window::ThemeMode;
use cushy::Run;

fn content() -> impl MakeWidget {
    let a = Progress::Percent(ZeroToOne::new(0.5));
    let b = Progress::Percent(ZeroToOne::ONE);

    let progress = Dynamic::new(a);

    progress.with_clone(|progress| {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(100));
            progress.set(b);
            thread::sleep(Duration::from_millis(100));
            progress.set(a);
        });
    });

    progress
        .clone()
        .progress_bar()
        .width(Px::new(100)) // Why is it not possible to use `Px::new(100)..`
        .contain()
        .centered()
        .expand()
        .contain()
}

pub fn main() -> cushy::Result<()> {
    content().themed_mode(ThemeMode::Dark).run()
}
