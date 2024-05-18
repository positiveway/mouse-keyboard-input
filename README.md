Easy to use `uinput` wrapper for Rust.

Allows you to send keyboard and mouse events by creating a virtual device in Linux.

`uinput` is a basic Linux library, so this works on any distro and on `X11` or `Wayland`.

High-resolution events are sent for the mouse wheel, allowing smoother scrolling and better precision.

Lib is safe by design, resources are released automatically when `VirtualDevice`'s destructor is called. Dependencies are up-to-date in contrast to other `uinput` libs for Rust.


## Installation
To use it without `sudo` add the current user to input group (replace `user` with your username):
```
sudo usermod -a -G input user
sudo reboot
```

### Libraries required

On Ubuntu and Debian:
```
sudo apt install libudev-dev libevdev-dev libhidapi-dev
```

Add to `Cargo.toml`
```
mouse-keyboard-input = "0.7.3"
```
To use the latest development version:
```
mouse-keyboard-input = { git = "https://github.com/positiveway/mouse-keyboard-input", branch = "main" }
```

#### armv7 build on linux
Build script can be found in [/scripts/build_armv7.sh](https://github.com/positiveway/mouse-keyboard-input/blob/main/scripts/build_armv7.sh)
```
cd ./scripts
sudo chmod +x ./build_armv7.sh #make script executable
./build_armv7.sh
```
For this to work `arm-linux-gnueabi-gcc` needs to be installed

For Ubuntu:
```
sudo apt install gcc-arm-linux-gnueabi
```
Reboot is required after installing the package

For more details read [official discussion](https://github.com/rust-lang/cargo/issues/11212)

## How to use
### Functions
```
click(button_or_key) - click mouse button or type a key
press(button_or_key)
release(button_or_key)

move_mouse(x, y)

scroll_vertical(value)
scroll_horizontal(value)
```
### List of buttons
#### Mouse
```
BTN_LEFT - left mouse button
BTN_RIGHT - right mouse button
BTN_MIDDLE - middle mouse button
```

#### Keyboard
Key codes can be found in [/src/key_codes.rs](https://github.com/positiveway/mouse-keyboard-input/blob/main/src/key_codes.rs)

Example:
```
KEY_A
KEY_LEFTSHIFT
KEY_LEFTMETA (Meta means Windows button on Linux)
```

### Code examples
#### Mouse
```
extern crate mouse_keyboard_input;

use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut device = VirtualDevice::new();

    for _ in 1..5 {
        thread::sleep(Duration::from_secs(1));

        // scroll down by 100
        device.scroll_vertical(-100).unwrap();
        // move cursor 50 pixels up and 50 pixels to the right from the current position
        device.move_mouse(50, 50).unwrap();
        //click the left mouse button
        device.click(BTN_LEFT).unwrap();
    }
}
```
#### Keyboard
```
extern crate mouse_keyboard_input;

use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;
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
```

### Sending events from multiple threads is also supported. See [/examples/channels.rs](https://github.com/positiveway/mouse-keyboard-input/blob/main/examples/channels.rs)

## Contributors
Based on [github.com/meh/rust-uinput](https://github.com/meh/rust-uinput)