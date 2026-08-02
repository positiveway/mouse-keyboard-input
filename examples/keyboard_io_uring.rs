#[cfg(not(feature = "io-uring"))]
fn main() {
    eprintln!("This test requires the 'io-uring' feature.");
    eprintln!("Please run with: cargo run --release --example keyboard --features io-uring");
}

#[cfg(feature = "io-uring")]
fn main() {
    use mouse_keyboard_input::VirtualDevice;
    use mouse_keyboard_input::key_codes::*;
    use std::thread;
    use std::time::Duration;

    println!("Initializing VirtualDevice with io_uring backend...");
    let mut device = VirtualDevice::default().unwrap();

    thread::sleep(Duration::from_secs(2));

    println!("Typing 'hello' (routed via async io_uring writes)...");
    // type hello
    for key in [KEY_H, KEY_E, KEY_L, KEY_L, KEY_O] {
        device.click(key).unwrap();
    }

    println!("Test complete. The Drop implementation will synchronously flush any remaining io_uring events before exit.");
}