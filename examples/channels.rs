use mouse_keyboard_input::*;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn write_events_in_thread(sender: ChannelSender) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 1..5 {
            thread::sleep(Duration::from_secs(1));

            // scroll vertically by 100
            VirtualDevice::send_scroll_y(100, &sender.clone()).unwrap();
            // move cursor vertically from the current position by 50
            VirtualDevice::send_mouse_move(50, 50, &sender.clone()).unwrap();
            //click the left mouse button
            VirtualDevice::send_click(BTN_LEFT, &sender.clone()).unwrap();
        };
    })
}

fn main() {
    let device = VirtualDevice::default().unwrap();

    let sender = device.sender.clone();

    device.flush_channel_every_interval();

    write_events_in_thread(sender).join().unwrap();
}