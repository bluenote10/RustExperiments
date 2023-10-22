use input_linux::{AbsoluteEvent, EvdevHandle, Event, EventTime, InputEvent};
use nix::libc::O_NONBLOCK;
use std::fs::OpenOptions;
use std::io::{self, ErrorKind};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
/// Resources on tweaking the polling rate:
/// - https://wiki.archlinux.org/title/Mouse_polling_rate
/// - https://superuser.com/questions/369826/increase-usb-polling-rate-across-all-devices-in-linux
///
/// Using `lsusb` and then `lsusb -vd 0079:0011` on the device ID shows that the polling
/// rate should already be 10 ms.
///
/// This can also be confirmed with:
/// - `sudo mount -t debugfs none /sys/kernel/debug` (https://en.wikipedia.org/wiki/Debugfs)
/// - `sudo less /sys/kernel/debug/usb/devices`
/// - and looking for the gamepad device there.

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

    let mut last_event_time: Option<(SystemTime, SystemTime)> = None;
    let verbose = false;

    let mut handle_button_down = |event: AbsoluteEvent| {
        let curr_event_time_external = SystemTime::now();
        let curr_event_time_internal = into_system_time(&event.time);
        if let Some((last_event_time_external, last_event_time_internal)) = last_event_time {
            let delta_external = curr_event_time_external
                .duration_since(last_event_time_external)
                .unwrap();
            let delta_internal = curr_event_time_internal
                .duration_since(last_event_time_internal)
                .unwrap();
            println!(
                "delta (external): {:8.3} ms    delta (internal): {:8.3} ms    button: {:?}",
                delta_external.as_secs_f64() * 1000.0,
                delta_internal.as_secs_f64() * 1000.0,
                event
            );
        }
        last_event_time = Some((curr_event_time_external, curr_event_time_internal));
    };

    loop {
        let event = read_event(&evdev_handle).unwrap();
        if let Some(event) = event {
            if verbose {
                println!("{:?}", event);
            }
            if let Event::Absolute(absolute_event) = event {
                if absolute_event.value == 0 || absolute_event.value == 255 {
                    handle_button_down(absolute_event)
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

fn into_system_time(event_time: &EventTime) -> SystemTime {
    UNIX_EPOCH
        + Duration::from_secs(event_time.seconds() as u64)
        + Duration::from_micros(event_time.microseconds() as u64)
}
