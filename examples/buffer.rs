extern crate mouse_keyboard_input;

use std::sync::{Arc, Mutex};
use mouse_keyboard_input::{FIXED_TIME, input_event, VirtualDevice};
use mouse_keyboard_input::key_codes::*;
use std::thread;
use std::time::Duration;


fn main() {
    let mut device = VirtualDevice::new();

    let SYNC_EVENT: input_event = input_event { kind: EV_SYN, code: SYN_REPORT, value: 0, time: FIXED_TIME };

    let instructions = Vec::new();

    let mut buffer = instructions;
    buffer.push(SYNC_EVENT);

    device.write_buffer(buffer.as_slice()).unwrap();
}