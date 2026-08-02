use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut device = VirtualDevice::default().unwrap();

    for _ in 1..3 {
        thread::sleep(Duration::from_secs(1));

        // gradually scroll down by 100
        device.smooth_scroll(0, -100).unwrap();
        // gradually move cursor 250 pixels up and 250 pixels to the right from the current position
        device.smooth_move_mouse(250, 250).unwrap();
        //click the left mouse button
        device.click(BTN_RIGHT).unwrap();
    }

    for _ in 1..2 {
        thread::sleep(Duration::from_secs(1));

        // scroll down by 100
        device.scroll_y(-100).unwrap();
        // instantly move cursor 250 pixels up and 250 pixels to the right from the current position
        device.move_mouse(250, 250).unwrap();
        //click the left mouse button
        device.click(BTN_RIGHT).unwrap();
    }
}