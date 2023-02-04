extern crate uinput_simple;

use uinput_simple::FakeDevice;
use uinput_simple::events::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut device = FakeDevice::new();
    for _ in 1..10 {
        thread::sleep(Duration::from_secs(1));

        device.move_mouse_or_wheel(REL_X, 50).unwrap();
        device.move_mouse_or_wheel(REL_Y, 50).unwrap();
        device.synchronize().unwrap();
    }
}