use std::path::Path;
use std::{mem, ptr, slice};
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;
use nix::errno::Errno;

use crate::*;

pub type Res<T> = Result<T, Error>;


pub struct VirtualDevice {
    file: File,
    def: uinput_user_dev,
    event: input_event,
}

const FIXED_TIME: timeval = timeval { tv_sec: 0, tv_usec: 0 };

impl VirtualDevice {
    pub fn new() -> Self {
        let path = Path::new("/dev/uinput");

        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path).unwrap();

        use std::os::unix::fs::PermissionsExt;

        let metadata = file.metadata().unwrap();
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

        let mut event = input_event {
            time: FIXED_TIME,
            kind: 0,
            code: 0,
            value: 0,
        };

        let mut virtual_device = VirtualDevice {
            file,
            def: unsafe { mem::zeroed() },
            event,
        };

        virtual_device.set_name("virtualdevice");
        virtual_device.register_all();
        virtual_device.create();

        return virtual_device;
    }

    fn set_name<T: AsRef<str>>(&mut self, value: T) {
        let string = CString::new(value.as_ref()).unwrap();
        let bytes = string.as_bytes_with_nul();

        (&mut self.def.name)[..bytes.len()]
            .clone_from_slice(unsafe { mem::transmute(bytes) });
    }

    fn create(&mut self) {
        unsafe {
            let ptr = &self.def as *const _ as *const u8;
            let size = mem::size_of_val(&self.def);

            self.file.write_all(slice::from_raw_parts(ptr, size)).unwrap();

            Errno::result(ui_dev_create(self.file.as_raw_fd())).unwrap();
        }
    }

    fn register_all(&self) {
        for code in 1..127 {
            self.register_key(code);
        }
        for code in [BTN_LEFT, BTN_RIGHT, BTN_MIDDLE] {
            self.register_key(code);
        }

        for code in [REL_X, REL_Y, REL_HWHEEL, REL_WHEEL] {
            self.register_relative(code);
        }
    }

    fn register_key(&self, code: u16) {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), EV_KEY as i32)).unwrap();
            Errno::result(ui_set_keybit(self.file.as_raw_fd(), code as i32)).unwrap();
        }
    }

    fn register_relative(&self, code: u16) {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), EV_REL as i32)).unwrap();
            Errno::result(ui_set_relbit(self.file.as_raw_fd(), code as i32)).unwrap();
        }
    }

    fn write(&mut self, kind: u16, code: u16, value: i32) -> Res<()> {
        self.event.kind = kind;
        self.event.code = code;
        self.event.value = value;

        unsafe {
            // gettimeofday(&mut event.time, ptr::null_mut());

            let ptr = &self.event as *const _ as *const u8;
            let size = mem::size_of_val(&self.event);

            self.file.write_all(slice::from_raw_parts(ptr, size)).unwrap();
        }
        Ok(())
    }

    pub fn synchronize(&mut self) -> Res<()> {
        self.write(EV_SYN, SYN_REPORT, 0)
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Res<()> {
        self.write(EV_REL, REL_X, x)?;
        self.write(EV_REL, REL_Y, y)?;
        self.synchronize()
    }

    pub fn scroll_vertical(&mut self, value: i32) -> Res<()> {
        self.write(EV_REL, REL_WHEEL, -value)?;
        self.synchronize()
    }

    pub fn scroll_horizontal(&mut self, value: i32) -> Res<()> {
        self.write(EV_REL, REL_HWHEEL, value)?;
        self.synchronize()
    }

    pub fn press(&mut self, button: u16) -> Res<()> {
        self.write(EV_KEY, button, 1)?;
        self.synchronize()
    }

    pub fn release(&mut self, button: u16) -> Res<()> {
        self.write(EV_KEY, button, 0)?;
        self.synchronize()
    }

    pub fn click(&mut self, button: u16) -> Res<()> {
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