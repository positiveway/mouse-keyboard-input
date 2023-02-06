extern crate uinput;

use uinput::VirtualDevice;
use uinput::events::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut device = VirtualDevice::new();

    for _ in 1..5 {
        thread::sleep(Duration::from_secs(1));

        // scroll vertically by 100
        device.scroll_vertical(100).unwrap();
        // move cursor vertically from current position by 50
        device.move_mouse(50, 50).unwrap();
        //click left mouse button
        device.click(BTN_LEFT).unwrap();
    }
}