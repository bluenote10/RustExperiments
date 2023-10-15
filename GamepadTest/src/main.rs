use std::time::{Duration, SystemTime};

use gilrs::{Button, Event, EventType, Gilrs};

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
}

impl StreakHandler {
    fn new() -> Self {
        Self { times: Vec::new() }
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

    fn report_streak(&self) {
        assert!(self.times.len() > 0);
        if self.times.len() == 1 {
            println!("Streak of length 1");
        } else {
            let first_time = *self.times.first().unwrap();
            let last_time = *self.times.last().unwrap();
            let delta = last_time.duration_since(first_time).unwrap();
            let frequency = self.times.len() as f64 / delta.as_secs_f64();
            println!(
                "Streak of length {}; total delta: {:?}; frequency: {:.3}",
                self.times.len(),
                delta,
                frequency
            );
        }
    }
}

fn main() {
    let verbose = false;

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
    }
}
