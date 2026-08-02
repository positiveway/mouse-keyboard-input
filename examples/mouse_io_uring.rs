#[cfg(not(feature = "io-uring"))]
fn main() {
    eprintln!("This test requires the 'io-uring' feature.");
    eprintln!("Please run with: cargo run --release --example mouse --features io-uring");
}

#[cfg(feature = "io-uring")]
fn main() {
    use mouse_keyboard_input::VirtualDevice;
    use mouse_keyboard_input::key_codes::*;
    use std::thread;
    use std::time::{Duration, Instant};

    println!("Initializing VirtualDevice with io_uring backend...");
    let mut device = VirtualDevice::default().unwrap();

    println!("Testing smooth operations (routed via async io_uring writes)...");
    for _ in 1..3 {
        thread::sleep(Duration::from_secs(1));

        // gradually scroll down by 100
        device.smooth_scroll(0, -100).unwrap();
        // gradually move cursor 250 pixels up and 250 pixels to the right from the current position
        device.smooth_move_mouse(250, 250).unwrap();
        //click the right mouse button
        device.click(BTN_RIGHT).unwrap();
    }

    println!("Testing instant operations (routed via async io_uring writes)...");
    for _ in 1..2 {
        thread::sleep(Duration::from_secs(1));

        // scroll down by 100
        device.scroll_y(-100).unwrap();
        // instantly move cursor 250 pixels up and 250 pixels to the right from the current position
        device.move_mouse(250, 250).unwrap();
        //click the right mouse button
        device.click(BTN_RIGHT).unwrap();
    }

    println!("Testing high-frequency event submission (where io_uring drastically reduces latency)...");
    // In a standard synchronous setup, 1000 syscalls would introduce significant jitter.
    // With io_uring, these are submitted asynchronously with zero syscalls per event.
    let start = Instant::now();
    let iterations = 1000;
    for _ in 0..iterations {
        // 1000 small 1-pixel movements submitted asynchronously
        device.move_mouse_raw(-1, 0).unwrap();
    }
    let elapsed = start.elapsed();

    println!("Successfully queued {} events in {:?} (avg {:?}/event)",
             iterations,
             elapsed,
             elapsed / iterations as u32);

    println!("Test complete. The Drop implementation will synchronously flush any remaining io_uring events before exit.");
}