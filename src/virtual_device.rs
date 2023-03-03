use std::path::Path;
use std::{mem, ptr, slice};
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use nix::errno::Errno;

use crate::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type EmptyResult = Result<()>;

pub type Button = u16;

pub struct VirtualDevice {
    file: File,
    def: uinput_user_dev,
    event: input_event,
    buffer: Vec<u8>,
}

const FIXED_TIME: timeval = timeval { tv_sec: 0, tv_usec: 0 };


const SLEEP_BEFORE_RELEASE: Duration = Duration::from_millis(5);

impl VirtualDevice {
    pub fn new() -> Result<Self> {
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

        let mut virtual_device = VirtualDevice {
            file,
            def: unsafe { mem::zeroed() },
            event,
            buffer: Vec::new(),
        };

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        let device_name = format!("virtualdevice-{}", now.as_millis());

        virtual_device.set_name(device_name.as_str());
        virtual_device.register_all()?;
        virtual_device.create()?;

        Ok(virtual_device)
    }

    fn set_name<T: AsRef<str>>(&mut self, value: T) {
        let string = CString::new(value.as_ref()).unwrap();
        let bytes = string.as_bytes_with_nul();

        (&mut self.def.name)[..bytes.len()]
            .clone_from_slice(unsafe { mem::transmute(bytes) });
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

    pub fn buffer_add_mouse_move(&mut self, x: i32, y: i32) {
        self.add_to_buffer(EV_REL, REL_X, x);
        self.add_to_buffer(EV_REL, REL_Y, y);
    }

    pub fn write_buffer_to_disk(&mut self) -> EmptyResult {
        self.buffer_add_sync();
        self.file.write_all(self.buffer.as_slice())?;
        self.buffer.clear();
        Ok(())
    }

    pub fn write_events_from_buffer(&mut self, buffer: &[input_event]) -> EmptyResult {
        let mut converted = Vec::new();

        for event in buffer.iter() {
            self.event.kind = event.kind;
            self.event.code = event.code;
            self.event.value = event.value;

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

    pub fn move_mouse_x(&mut self, x: i32) -> EmptyResult {
        self.write(EV_REL, REL_X, x)?;
        self.synchronize()
    }

    pub fn move_mouse_y(&mut self, y: i32) -> EmptyResult {
        self.write(EV_REL, REL_Y, y)?;
        self.synchronize()
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> EmptyResult {
        self.write(EV_REL, REL_X, x)?;
        self.write(EV_REL, REL_Y, y)?;
        self.synchronize()
    }

    pub fn scroll_vertical(&mut self, value: i32) -> EmptyResult {
        self.write(EV_REL, REL_WHEEL, -value)?;
        self.synchronize()
    }

    pub fn scroll_horizontal(&mut self, value: i32) -> EmptyResult {
        self.write(EV_REL, REL_HWHEEL, value)?;
        self.synchronize()
    }

    pub fn press(&mut self, button: u16) -> EmptyResult {
        self.write(EV_KEY, button, 1)?;
        self.synchronize()
    }

    pub fn release(&mut self, button: u16) -> EmptyResult {
        sleep(SLEEP_BEFORE_RELEASE);

        self.write(EV_KEY, button, 0)?;
        self.synchronize()
    }

    pub fn click(&mut self, button: u16) -> EmptyResult {
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