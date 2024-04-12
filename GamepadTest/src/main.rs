use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::{Duration, SystemTime};

use colored::Colorize;
use gilrs::{Button, Event, EventType, Gilrs};
use rodio::{source::Source, Decoder, OutputStream};

fn is_dpad_event(event_type: &EventType) -> bool {
    match event_type {
        EventType::ButtonChanged(button, value, _code)
            if (*value == 0.0 || *value == 1.0) && *button == Button::DPadRight =>
        {
            true
        }
        _ => false,
    }
}

struct StreakHandler {
    times: Vec<SystemTime>,
    best_deltas: HashMap<usize, f64>,
}

impl StreakHandler {
    fn new() -> Self {
        Self {
            times: Vec::new(),
            best_deltas: HashMap::new(),
        }
    }

    fn handle_loop(&mut self, was_event: bool) {
        let curr_time = SystemTime::now();

        if let Some(&last_time) = self.times.last() {
            let delta = curr_time.duration_since(last_time).unwrap();
            if delta > Duration::from_millis(300) {
                self.report_streak();
                self.times.clear();
            }
        }

        if was_event {
            self.times.push(curr_time);
        }
    }

    fn report_streak(&mut self) {
        assert!(self.times.len() > 0);
        if self.times.len() == 1 {
            println!("Streak {}", format!("1").bold().bright_blue());
        } else {
            let first_time = *self.times.first().unwrap();
            let last_time = *self.times.last().unwrap();
            let delta = last_time.duration_since(first_time).unwrap();
            let frequency = self.times.len() as f64 / delta.as_secs_f64();

            let entry = self.best_deltas.entry(self.times.len());
            let is_new_best = match entry {
                Entry::Occupied(mut entry) => {
                    let value = entry.get_mut();
                    if delta.as_secs_f64() < *value {
                        *value = delta.as_secs_f64();
                        true
                    } else {
                        false
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(delta.as_secs_f64());
                    true
                }
            };

            println!(
                "Streak {}    total delta: {:6.1} ms     avg delta: {:6.1} ms   frequency: {:5.2} hz{}",
                format!("{}", self.times.len()).bold().bright_blue(),
                delta.as_secs_f64() * 1000.0,
                delta.as_secs_f64() / self.times.len() as f64 * 1000.0,
                frequency,
                if is_new_best {
                    " [NEW BEST]".bold().bright_green()
                } else {
                    "".normal()
                }
            );
        }
    }
}

fn main() {
    let verbose = false;

    // Audio source: https://freesound.org/people/Breviceps/sounds/447910/
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("plop.wav").unwrap());
    let source = Decoder::new(file).unwrap().buffered();
    source.raw_audio

    let mut gilrs = Gilrs::new().unwrap();

    for (id, gamepad) in gilrs.gamepads() {
        println!(
            "Gamepad: id = {}, name = {}, power_info = {:?}",
            id,
            gamepad.name(),
            gamepad.power_info()
        );
    }

    let mut last_time: Option<SystemTime> = None;
    let mut streak_handler = StreakHandler::new();

    loop {
        let is_dpad_event = if let Some(Event {
            id: gamepad_id,
            event: event_type,
            time,
        }) = gilrs.next_event()
        {
            if verbose {
                println!("{:?} New event from {}: {:?}", time, gamepad_id, event_type);
            }
            let is_dpad_event = is_dpad_event(&event_type);
            if is_dpad_event {
                if verbose {
                    println!("Detected dpad event");
                    if let Some(last_time) = last_time {
                        let delta = time.duration_since(last_time).unwrap();
                        println!("{:?}", delta);
                    }
                }
                last_time = Some(time);
            }
            is_dpad_event
        } else {
            false
        };

        streak_handler.handle_loop(is_dpad_event);

        if is_dpad_event {
            stream_handle
                .play_raw(source.clone().convert_samples())
                .unwrap();
        }
    }
}
