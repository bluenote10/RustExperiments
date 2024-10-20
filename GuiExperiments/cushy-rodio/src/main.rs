use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use cushy::animation::ZeroToOne;
use cushy::figures::units::Lp;
use cushy::figures::Zero;
use cushy::value::{Destination, Switchable};
use cushy::value::{Dynamic, IntoReader};
use cushy::widget::{MakeWidget, Widget};
use cushy::widgets::progress::{Progress, Progressable};
use cushy::Run;
use rodio::source::{SineWave, Source};
use rodio::{OutputStream, OutputStreamHandle, Sink};

#[derive(Debug, Default, Eq, PartialEq)]
struct Task {
    progress: Dynamic<Progress>,
}

fn main() -> cushy::Result {
    let task = Dynamic::new(None::<Task>);

    task.switcher(|task, dynamic| {
        if let Some(task) = task {
            // A background thread is running, show a progress bar.
            task.progress.clone().progress_bar().make_widget()
        } else {
            // There is no background task. Show a button that will start one.
            "▶"
                .into_button()
                .on_click({
                    let task = dynamic.clone();
                    move |_| {
                        let background_task = Task::default();
                        spawn_background_thread(&background_task.progress, &task);
                        task.set(Some(background_task));
                    }
                })
                .make_widget()
        }
    })
    .and(SinePlayer())
    .into_rows()
    .contain()
    .centered()
    .run()
}

fn spawn_background_thread(progress: &Dynamic<Progress>, task: &Dynamic<Option<Task>>) {
    let progress = progress.clone();
    let task = task.clone();
    std::thread::spawn(move || background_task(&progress, &task));
}

fn background_task(progress: &Dynamic<Progress>, task: &Dynamic<Option<Task>>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(2.0))
        .amplify(0.20);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();

    for i in 0_u8..=10 {
        progress.set(Progress::Percent(ZeroToOne::new(f32::from(i) / 10.)));
        std::thread::sleep(Duration::from_millis(100));
    }
    task.set(None);
}

/*
#[derive(Clone)]
struct Player {
    stream: Arc<Mutex<OutputStream>>,
    sink: Arc<Mutex<Sink>>,
}

impl Player {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
        Self {
            stream: Arc::new(Mutex::new(stream)),
            sink,
        }
    }
}

*/

thread_local! {
    static STREAM: RefCell<Option<(OutputStream, OutputStreamHandle)>> = RefCell::new(None);
}

fn get_output_stream_handle() -> OutputStreamHandle {
    STREAM.with_borrow_mut(|stream_tup| {
        if let Some((_stream, stream_handle)) = stream_tup {
            stream_handle.clone()
        } else {
            let (stream, stream_handle) = OutputStream::try_default().unwrap();
            *stream_tup = Some((stream, stream_handle.clone()));
            stream_handle
        }
    })
}

fn SinePlayer() -> impl Widget {
    let progress = Dynamic::new(Progress::Percent(ZeroToOne::ZERO));

    let sink = Arc::new(Mutex::new(
        Sink::try_new(&get_output_stream_handle()).unwrap(),
    ));

    "▶"
        .into_button()
        .on_click({
            let progress = progress.clone();
            let sink = sink.clone();
            move |_| {
                spawn_background_thread2(&progress, sink.clone());
                // task.set(Some(background_task));
            }
        })
        .make_widget()
        .and(progress.clone().progress_bar().make_widget())
        .into_columns()
        .contain()
        .centered()
}

fn spawn_background_thread2(progress: &Dynamic<Progress>, sink: Arc<Mutex<Sink>>) {
    let progress = progress.clone();
    std::thread::spawn(move || background_task2(&progress, sink));
}

fn background_task2(progress: &Dynamic<Progress>, sink: Arc<Mutex<Sink>>) {
    let sink = sink.lock().unwrap();

    let length = 2.0;
    let source = SineWave::new(440.0)
        .take_duration(Duration::from_secs_f32(length))
        .amplify(0.20);
    sink.append(source);

    // sink.sleep_until_end();

    println!("Start playing...");

    while !sink.empty() {
        let pos = sink.get_pos();
        println!("pos: {:?}", pos);
        progress.set(Progress::Percent(ZeroToOne::new(
            pos.as_secs_f32() / length,
        )));
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Finished playing...");
}

fn main2() -> cushy::Result {
    let counter = Dynamic::new(0i32);

    counter
        .to_label()
        .width(Lp::points(100))
        .and("+".into_button().on_click(counter.with_clone(|counter| {
            move |_| {
                *counter.lock() += 1;
            }
        })))
        .and("-".into_button().on_click(counter.with_clone(|counter| {
            move |_| {
                *counter.lock() -= 1;
            }
        })))
        .into_columns()
        .centered()
        .run()
}
