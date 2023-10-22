use input_linux::{AbsoluteEvent, EvdevHandle, Event, InputEvent};
use nix::libc::O_NONBLOCK;
use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::SystemTime;

///
/// Inspired by:
///
/// - https://github.com/gorbit99/olcPGEX_Gamepad/blob/master/olcPGEX_Gamepad.h
/// - https://github.com/arcnmx/input-linux-rs/blob/main/examples/mouse-movements.rs
///
/// Note that all these ioctl calls with `EVIOCG*` codes should have abstractions
/// in the linux-input crate, so we shouldn't have to deal with them directly:
/// - https://docs.rs/input-linux/latest/input_linux/evdev/struct.EvdevHandle.html
///
/// Apparently Linux has a deprecated "joydev" and "evdev" system. The ID "/dev/input/js3"
/// seems to be the joydev part of my gamepad. This doesn't work with the evdev API. The
/// evdev API needs the corresponding "eventXX" file descriptor. It is easiest to find the
/// sibling by looking at `ll /dev/input/by-id`.
///
/// The "joydev vs evdev" confusion was mentioned here:
/// - https://technicallycompetent.com/joysticks-linux-joydev-evdev/
/// - https://wiki.archlinux.org/title/Gamepad
///

fn main() -> io::Result<()> {
    let path = Path::new("/dev/input/event14"); // "/dev/input/js3"
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(O_NONBLOCK)
        .open(&path)?;

    let fd = file.as_raw_fd();
    println!("{}", fd);

    // This is problematic, because we may drop the underlying file handle while it
    // is still in use.
    // let evdev_handle = unsafe { EvdevHandle::from_raw_fd(fd) };
    let evdev_handle = EvdevHandle::new(file);

    println!("Device id: {:?}", evdev_handle.device_id());
    println!(
        "Device name: {:?}",
        std::str::from_utf8(&evdev_handle.device_name().unwrap()).unwrap()
    );
    println!("Driver version: {:?}", evdev_handle.driver_version());
    println!(
        "Physical location: {:?}",
        std::str::from_utf8(&evdev_handle.physical_location().unwrap()).unwrap()
    );

    let mut last_event_time: Option<SystemTime> = None;
    let verbose = false;

    let mut handle_button_down = |event: Event| {
        let curr_event_time = SystemTime::now();
        if let Some(last_event_time) = last_event_time {
            let delta = curr_event_time.duration_since(last_event_time).unwrap();
            println!(
                "delta: {:8.3} ms    button: {:?}",
                delta.as_secs_f64() * 1000.0,
                event
            );
        }
        last_event_time = Some(curr_event_time);
    };

    loop {
        let event = read_event(&evdev_handle).unwrap();
        if let Some(event) = event {
            if verbose {
                println!("{:?}", event);
            }
            if let Event::Absolute(absolute) = event {
                if absolute.value == 0 || absolute.value == 255 {
                    handle_button_down(event)
                }
            }
        }
    }
}

fn read_event<F>(evdev_handle: &EvdevHandle<F>) -> io::Result<Option<Event>>
where
    F: AsRawFd,
{
    let mut events = [input_linux::sys::input_event {
        time: input_linux::sys::timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        type_: 0,
        code: 0,
        value: 0,
    }];
    let result = evdev_handle.read(&mut events);
    match result {
        Ok(count) => {
            // We should have read exactly 1 event here.
            assert_eq!(count, 1);
            Ok(Some(
                Event::new(InputEvent::from_raw(&events[0]).unwrap().clone()).unwrap(),
            ))
        }
        // The 'WouldBlock' error is entirely expected when polling a file description in O_NONBLOCK
        // mode, so it makes more sense to map it to `None` (no event available).
        Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
        Err(err) => Err(err),
    }
}
