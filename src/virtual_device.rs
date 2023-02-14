use std::path::Path;
use std::{mem, ptr, slice};
use libc::{timeval, gettimeofday};
use std::ffi::CString;
use nix::{fcntl, unistd};
use nix::errno::Errno;
use nix::sys::stat;

use crate::*;

pub type Res<T> = Result<T, Error>;


pub struct VirtualDevice {
    fd: i32,
    def: uinput_user_dev,
}

const FIXED_TIME: timeval = timeval { tv_sec: 0, tv_usec: 0 };

impl VirtualDevice {
    fn open<P: AsRef<Path>>(path: P) -> Res<Self> {
        // let usb_device = input_id {
        //     bustype: 0x03,
        //     vendor: 0x4711,
        //     product: 0x0816,
        //     version: 1,
        // };
        // let mut def: uinput_user_dev = unsafe { mem::zeroed() };
        // def.id = usb_device;

        Ok(VirtualDevice {
            fd: fcntl::open(path.as_ref(), fcntl::OFlag::O_WRONLY | fcntl::OFlag::O_NONBLOCK, stat::Mode::S_IRUSR | stat::Mode::S_IWUSR | stat::Mode::S_IRGRP | stat::Mode::S_IWGRP).unwrap(),
            def: unsafe { mem::zeroed() },
        })
    }

    pub fn new() -> Self {
        let path = Path::new("/dev/uinput");

        let mut virtual_device = VirtualDevice::open(path).unwrap();

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

    fn create(&self) {
        unsafe {
            let ptr = &self.def as *const _ as *const u8;
            let size = mem::size_of_val(&self.def);

            unistd::write(self.fd, slice::from_raw_parts(ptr, size)).unwrap();
            Errno::result(ui_dev_create(self.fd)).unwrap();
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
            Errno::result(ui_set_evbit(self.fd, EV_KEY as i32)).unwrap();
            Errno::result(ui_set_keybit(self.fd, code as i32)).unwrap();
        }
    }

    fn register_relative(&self, code: u16) {
        unsafe {
            Errno::result(ui_set_evbit(self.fd, EV_REL as i32)).unwrap();
            Errno::result(ui_set_relbit(self.fd, code as i32)).unwrap();
        }
    }

    fn write(&mut self, kind: u16, code: u16, value: i32) -> Res<()> {
        unsafe {
            let mut event = input_event {
                time: FIXED_TIME,
                kind,
                code,
                value,
            };

            // gettimeofday(&mut event.time, ptr::null_mut());

            let ptr = &event as *const _ as *const u8;
            let size = mem::size_of_val(&event);

            unistd::write(self.fd, slice::from_raw_parts(ptr, size))?;
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
            ui_dev_destroy(self.fd);
        }
    }
}