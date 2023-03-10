use std::path::Path;
use std::{mem, ptr, slice};
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use nix::errno::Errno;
use crossbeam_channel::{bounded, Sender, Receiver};

use crate::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type EmptyResult = Result<()>;

pub type Button = u16;
pub type Coord = i32;

pub type EventParams = (u16, u16, i32);
pub type ChannelSender = Sender<EventParams>;
type ChannelReceiver = Receiver<EventParams>;

pub struct VirtualDevice {
    file: File,
    def: uinput_user_dev,
    event: input_event,
    buffer: Vec<u8>,
    pub sender: ChannelSender,
    receiver: ChannelReceiver,
}

const FIXED_TIME: timeval = timeval { tv_sec: 0, tv_usec: 0 };
const SYN_PARAMS: EventParams = (EV_SYN, SYN_REPORT, 0);

const SLEEP_BEFORE_RELEASE: Duration = Duration::from_millis(5);


pub fn send_to_channel(kind: u16, code: u16, value: i32, sender: ChannelSender) -> EmptyResult {
    sender.send((kind, code, value))?;
    Ok(())
}

pub fn send_press(button: Button, sender: ChannelSender) -> EmptyResult {
    sender.send((EV_KEY, button, 1))?;
    sender.send(SYN_PARAMS)?;
    Ok(())
}

pub fn send_release(button: Button, sender: ChannelSender) -> EmptyResult {
    sender.send((EV_KEY, button, 0))?;
    Ok(())
}

pub fn send_mouse_move(x: Coord, y: Coord, sender: ChannelSender) -> EmptyResult {
    sender.send((EV_REL, REL_X, x))?;
    sender.send((EV_REL, REL_Y, y))?;
    Ok(())
}

pub fn send_scroll_vertical(value: Coord, sender: ChannelSender) -> EmptyResult {
    sender.send((EV_REL, REL_WHEEL, -value))?;
    Ok(())
}

impl VirtualDevice {
    pub fn default() -> Result<Self> {
        VirtualDevice::new(50)
    }
    pub fn new(channel_size: usize) -> Result<Self> {
        let path = Path::new("/dev/uinput");

        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)?;

        use std::os::unix::fs::PermissionsExt;

        let metadata = file.metadata()?;
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o660);

        // let usb_device = input_id {
        //     bustype: 0x03,
        //     vendor: 0x4711,
        //     product: 0x0816,
        //     version: 1,
        // };
        // let mut def: uinput_user_dev = unsafe { mem::zeroed() };
        // def.id = usb_device;

        let event = input_event {
            time: FIXED_TIME,
            kind: 0,
            code: 0,
            value: 0,
        };

        let (s, r) = bounded(channel_size);

        let mut virtual_device = VirtualDevice {
            file,
            def: unsafe { mem::zeroed() },
            event,
            buffer: Vec::new(),
            sender: s,
            receiver: r,
        };

        // let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        // let device_name = format!("virtualdevice-{}", now.as_millis());

        let device_name = String::from("virtualdevice");

        virtual_device.set_name(device_name.as_str())?;
        virtual_device.register_all()?;
        virtual_device.create()?;

        Ok(virtual_device)
    }

    fn set_name<T: AsRef<str>>(&mut self, value: T) -> EmptyResult {
        let string = CString::new(value.as_ref())?;
        let bytes = string.as_bytes_with_nul();

        (&mut self.def.name)[..bytes.len()]
            .clone_from_slice(unsafe { mem::transmute(bytes) });

        Ok(())
    }

    fn create(&mut self) -> EmptyResult {
        unsafe {
            let ptr = &self.def as *const _ as *const u8;
            let size = mem::size_of_val(&self.def);

            self.file.write_all(slice::from_raw_parts(ptr, size))?;

            Errno::result(ui_dev_create(self.file.as_raw_fd()))?;
        }
        Ok(())
    }

    fn register_all(&self) -> EmptyResult {
        for code in 1..127 {
            self.register_key(code)?
        }
        for code in [BTN_LEFT, BTN_RIGHT, BTN_MIDDLE] {
            self.register_key(code)?
        }

        for code in [REL_X, REL_Y, REL_HWHEEL, REL_WHEEL] {
            self.register_relative(code)?
        }
        Ok(())
    }

    fn register_key(&self, code: u16) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), EV_KEY as i32))?;
            Errno::result(ui_set_keybit(self.file.as_raw_fd(), code as i32))?;
        }
        Ok(())
    }

    fn register_relative(&self, code: u16) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), EV_REL as i32))?;
            Errno::result(ui_set_relbit(self.file.as_raw_fd(), code as i32))?;
        }
        Ok(())
    }

    fn add_to_buffer(&mut self, kind: u16, code: u16, value: i32) {
        self.event.kind = kind;
        self.event.code = code;
        self.event.value = value;

        unsafe {
            let ptr = &self.event as *const _ as *const u8;
            let size = mem::size_of_val(&self.event);
            let content = slice::from_raw_parts(ptr, size);

            self.buffer.extend_from_slice(content);
        }
    }

    pub fn buffer_add_sync(&mut self) {
        self.add_to_buffer(EV_SYN, SYN_REPORT, 0);
    }

    pub fn buffer_add_press(&mut self, button: Button) {
        self.add_to_buffer(EV_KEY, button, 1);
        self.buffer_add_sync();
    }

    pub fn buffer_add_release(&mut self, button: Button) {
        self.add_to_buffer(EV_KEY, button, 0);
    }

    pub fn buffer_add_click(&mut self, button: Button) {
        self.buffer_add_press(button);
        self.buffer_add_release(button);
    }

    pub fn buffer_add_mouse_move(&mut self, x: Coord, y: Coord) {
        self.add_to_buffer(EV_REL, REL_X, x);
        self.add_to_buffer(EV_REL, REL_Y, y);
    }

    pub fn buffer_add_scroll_vertical(&mut self, value: Coord) {
        self.add_to_buffer(EV_REL, REL_WHEEL, -value);
    }

    pub fn write_buffer_to_disk(&mut self) -> EmptyResult {
        self.buffer_add_sync();
        self.file.write_all(self.buffer.as_slice())?;
        self.buffer.clear();
        Ok(())
    }

    pub fn write_events_from_channel_buffered(&mut self) -> EmptyResult {
        let mut events_buffer = Vec::new();

        for event in self.receiver.try_iter() {
            events_buffer.push(event);
        }
        events_buffer.push(SYN_PARAMS);

        let mut converted = Vec::new();

        for event in events_buffer.iter() {
            self.event.kind = event.0;
            self.event.code = event.1;
            self.event.value = event.2;

            unsafe {
                let ptr = &self.event as *const _ as *const u8;
                let size = mem::size_of_val(&self.event);
                let content = slice::from_raw_parts(ptr, size);

                converted.extend_from_slice(content);
            }
        }
        let conv = converted.as_slice();

        self.file.write_all(conv)?;

        Ok(())
    }

    pub fn write_events_from_channel(&mut self) -> EmptyResult {
        let mut converted = Vec::new();
        self.sender.send(SYN_PARAMS)?;

        for event in self.receiver.try_iter() {
            self.event.kind = event.0;
            self.event.code = event.1;
            self.event.value = event.2;

            unsafe {
                let ptr = &self.event as *const _ as *const u8;
                let size = mem::size_of_val(&self.event);
                let content = slice::from_raw_parts(ptr, size);

                converted.extend_from_slice(content);
            }
        }
        let conv = converted.as_slice();

        self.file.write_all(conv)?;
        Ok(())
    }

    fn write(&mut self, kind: u16, code: u16, value: i32) -> EmptyResult {
        self.event.kind = kind;
        self.event.code = code;
        self.event.value = value;

        unsafe {
            // gettimeofday(&mut event.time, ptr::null_mut());

            let ptr = &self.event as *const _ as *const u8;
            let size = mem::size_of_val(&self.event);

            self.file.write_all(slice::from_raw_parts(ptr, size))?
        }
        Ok(())
    }

    pub fn synchronize(&mut self) -> EmptyResult {
        self.write(EV_SYN, SYN_REPORT, 0)
    }

    pub fn move_mouse_x(&mut self, x: Coord) -> EmptyResult {
        self.write(EV_REL, REL_X, x)?;
        self.synchronize()
    }

    pub fn move_mouse_y(&mut self, y: Coord) -> EmptyResult {
        self.write(EV_REL, REL_Y, y)?;
        self.synchronize()
    }

    pub fn move_mouse(&mut self, x: Coord, y: Coord) -> EmptyResult {
        self.write(EV_REL, REL_X, x)?;
        self.write(EV_REL, REL_Y, y)?;
        self.synchronize()
    }

    pub fn scroll_vertical(&mut self, value: Coord) -> EmptyResult {
        self.write(EV_REL, REL_WHEEL, -value)?;
        self.synchronize()
    }

    pub fn scroll_horizontal(&mut self, value: Coord) -> EmptyResult {
        self.write(EV_REL, REL_HWHEEL, value)?;
        self.synchronize()
    }

    pub fn press(&mut self, button: Button) -> EmptyResult {
        self.write(EV_KEY, button, 1)?;
        self.synchronize()
    }

    pub fn release(&mut self, button: Button) -> EmptyResult {
        sleep(SLEEP_BEFORE_RELEASE);

        self.write(EV_KEY, button, 0)?;
        self.synchronize()
    }

    pub fn click(&mut self, button: Button) -> EmptyResult {
        self.press(button)?;
        self.release(button)
    }
}

impl Drop for VirtualDevice {
    fn drop(&mut self) {
        unsafe {
            ui_dev_destroy(self.file.as_raw_fd());
        }
    }
}