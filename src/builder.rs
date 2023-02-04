use std::path::Path;
use std::{mem, ptr, slice};
use libc::{timeval, gettimeofday};
use std::ffi::CString;
use nix::{self, fcntl, unistd};
use nix::errno::Errno;
use nix::sys::stat;

use crate::error::*;
use crate::*;

pub type Res<T> = Result<T, Error>;


pub struct FakeDevice {
    fd: i32,
    def: uinput_user_dev,
}


impl FakeDevice {
    fn open<P: AsRef<Path>>(path: P) -> Res<Self> {
        Ok(FakeDevice {
            fd: fcntl::open(path.as_ref(), fcntl::OFlag::O_WRONLY | fcntl::OFlag::O_NONBLOCK, stat::Mode::empty()).unwrap(),
            def: unsafe { mem::zeroed() },
        })
    }

    pub fn new() -> Self {
        let context = udev::Context::new().unwrap();
        let mut enumerator = udev::Enumerator::new(&context).unwrap();

        enumerator.match_subsystem("misc").unwrap();
        enumerator.match_sysname("uinput").unwrap();

        let device = enumerator.scan_devices().unwrap()
            .next().unwrap();

        let path = device.devnode().unwrap();

        let mut fake_device = FakeDevice::open(path).unwrap();

        fake_device.set_name("testdevice");
        fake_device.register_all();
        fake_device.create();

        return fake_device;
    }

    /// Set the name.
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

    fn register_key(&self, code: i32) {
        unsafe {
            Errno::result(ui_set_evbit(self.fd, EV_KEY)).unwrap();
            Errno::result(ui_set_keybit(self.fd, code)).unwrap();
        }
    }

    fn register_relative(&self, code: i32) {
        unsafe {
            Errno::result(ui_set_evbit(self.fd, EV_REL)).unwrap();
            Errno::result(ui_set_relbit(self.fd, code)).unwrap();
        }
    }

    fn write(&mut self, kind: i32, code: i32, value: i32) -> Res<()> {
        unsafe {
            let mut event = input_event {
                time: timeval { tv_sec: 0, tv_usec: 0 },
                kind: kind as u16,
                code: code as u16,
                value: value as i32,
            };

            gettimeofday(&mut event.time, ptr::null_mut());

            let ptr = &event as *const _ as *const u8;
            let size = mem::size_of_val(&event);

            unistd::write(self.fd, slice::from_raw_parts(ptr, size))?;
        }

        Ok(())
    }

    pub fn synchronize(&mut self) -> Res<()> {
        self.write(EV_SYN, SYN_REPORT, 0)
    }

    pub fn move_mouse_or_wheel(&mut self, code: i32, value: i32) -> Res<()> {
        self.write(EV_REL, code, value)
    }

    pub fn press(&mut self, button: i32) -> Res<()> {
        self.write(EV_KEY, button, 1)
    }

    pub fn release(&mut self, button: i32) -> Res<()> {
        self.write(EV_KEY, button, 0)
    }
}

impl Drop for FakeDevice {
    fn drop(&mut self) {
        unsafe {
            ui_dev_destroy(self.fd);
        }
    }
}