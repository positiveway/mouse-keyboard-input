extern crate mouse_keyboard_input;

use std::sync::{Arc, Mutex};
use mouse_keyboard_input::{VirtualDevice};
use mouse_keyboard_input::key_codes::*;
use std::{mem, slice, thread};
use std::time::Duration;

fn main() {
    let mut device = VirtualDevice::default().unwrap();
    device.buffer_add_click(BTN_RIGHT);
    device.write_buffer_to_disk().unwrap();
}