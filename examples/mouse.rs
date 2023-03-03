extern crate mouse_keyboard_input;

use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut device = VirtualDevice::default().unwrap();

    for _ in 1..5 {
        thread::sleep(Duration::from_secs(1));

        // scroll vertically by 100
        device.scroll_vertical(100).unwrap();
        // move cursor vertically from the current position by 50
        device.move_mouse(50, 50).unwrap();
        //click the left mouse button
        device.click(BTN_LEFT).unwrap();
    }
}