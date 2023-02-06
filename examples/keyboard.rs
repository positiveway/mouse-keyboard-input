extern crate uinput;

use uinput::VirtualDevice;
use uinput::events::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut device = VirtualDevice::new();

    thread::sleep(Duration::from_secs(2));

    // type hello
    for key in [KEY_H, KEY_E, KEY_L, KEY_L, KEY_O] {
        device.click(key).unwrap();
    }
}