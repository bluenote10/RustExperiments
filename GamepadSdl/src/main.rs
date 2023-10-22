use std::time::SystemTime;

use sdl2::{controller::Button, event::Event};

///
/// Inspired by:
/// https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/game-controller.rs
///
/// Note that it may actually be better to call SDL_PumpEvents once per frame instead
/// of calling SDL_PollEvents multiple times:
/// - https://github.com/libsdl-org/SDL/issues/4376
///

fn main() -> Result<(), String> {
    // This is required for certain controllers to work on Windows without the
    // video subsystem enabled:
    sdl2::hint::set("SDL_JOYSTICK_THREAD", "1");

    let sdl_context = sdl2::init()?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let mut event_pump = sdl_context.event_pump()?;

    let available = game_controller_subsystem
        .num_joysticks()
        .map_err(|e| format!("can't enumerate joysticks: {}", e))?;

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    let controller = (0..available)
        .find_map(|id| {
            if !game_controller_subsystem.is_game_controller(id) {
                println!("{} is not a game controller", id);
                return None;
            }

            println!("Attempting to open controller {}", id);

            match game_controller_subsystem.open(id) {
                Ok(c) => {
                    // We managed to find and open a game controller,
                    // exit the loop
                    println!("Success: opened \"{}\"", c.name());
                    Some(c)
                }
                Err(e) => {
                    println!("failed: {:?}", e);
                    None
                }
            }
        })
        .expect("Couldn't open any controller");

    println!("Controller mapping: {}", controller.mapping());

    let mut last_event_time: Option<SystemTime> = None;
    let verbose = false;
    let use_busy_loop = true;

    let mut handle_button_down = |button: Button| {
        let curr_event_time = SystemTime::now();
        if let Some(last_event_time) = last_event_time {
            let delta = curr_event_time.duration_since(last_event_time).unwrap();
            println!(
                "delta: {:8.3} ms    button: {:?} down",
                delta.as_secs_f64() * 1000.0,
                button
            );
        }
        last_event_time = Some(curr_event_time);
    };

    if use_busy_loop {
        loop {
            let event = event_pump.poll_event();
            match event {
                Some(Event::ControllerButtonDown { button, .. }) => {
                    if verbose {
                        println!("Button {:?} down", button);
                    }
                    handle_button_down(button);
                }
                Some(Event::Quit { .. }) => break,
                _ => (),
            }
        }
    }
    for event in event_pump.wait_iter() {
        match event {
            Event::ControllerButtonDown { button, .. } => {
                if verbose {
                    println!("Button {:?} down", button);
                }
                handle_button_down(button);
            }
            Event::ControllerButtonUp { button, .. } => {
                if verbose {
                    println!("Button {:?} up", button);
                }
            }
            Event::Quit { .. } => break,
            _ => (),
        }
    }

    Ok(())
}
