use std::path::Path;
use std::{mem, ptr, slice, thread};
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use nix::errno::Errno;
use crossbeam_channel::{bounded, Sender, Receiver};

use crate::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type EmptyResult = Result<()>;

pub type Button = u16;
pub type Coord = i32;

pub type EventParams = (u16, u16, i32);
pub type ChannelSender = Sender<EventParams>;
type ChannelReceiver = Receiver<EventParams>;

pub struct VirtualDevice {
    writing_interval: Duration,
    file: File,
    def: uinput_user_dev,
    pub sender: ChannelSender,
    receiver: ChannelReceiver,
}

const FIXED_TIME: timeval = timeval { tv_sec: 0, tv_usec: 0 };
const SYN_PARAMS: EventParams = (EV_SYN, SYN_REPORT, 0);

const SLEEP_BEFORE_RELEASE: Duration = Duration::from_millis(5);


#[inline]
pub fn send_to_channel(kind: u16, code: u16, value: i32, sender: &ChannelSender) -> EmptyResult {
    sender.send((kind, code, value))?;
    Ok(())
}

#[inline]
pub fn send_press(button: Button, sender: &ChannelSender) -> EmptyResult {
    sender.send((EV_KEY, button, 1))?;
    sender.send(SYN_PARAMS)?;
    Ok(())
}

#[inline]
pub fn send_release(button: Button, sender: &ChannelSender) -> EmptyResult {
    sender.send((EV_KEY, button, 0))?;
    Ok(())
}

#[inline]
pub fn send_mouse_move_x(x: Coord, sender: &ChannelSender) -> EmptyResult {
    sender.send((EV_REL, REL_X, x))?;
    Ok(())
}

#[inline]
pub fn send_mouse_move_y(y: Coord, sender: &ChannelSender) -> EmptyResult {
    sender.send((EV_REL, REL_Y, y))?;
    Ok(())
}

#[inline]
pub fn send_mouse_move(x: Coord, y: Coord, sender: &ChannelSender) -> EmptyResult {
    sender.send((EV_REL, REL_X, x))?;
    sender.send((EV_REL, REL_Y, y))?;
    Ok(())
}

#[inline]
pub fn send_scroll_x(value: Coord, sender: &ChannelSender) -> EmptyResult {
    sender.send((EV_REL, REL_HWHEEL, -value))?;
    Ok(())
}

#[inline]
pub fn send_scroll_y(value: Coord, sender: &ChannelSender) -> EmptyResult {
    sender.send((EV_REL, REL_WHEEL, -value))?;
    Ok(())
}

#[inline]
fn convert_event_for_writing(kind: u16, code: u16, value: i32) -> &'static [u8] {
    // gettimeofday(&mut event.time, ptr::null_mut());

    let input_event = input_event {
        time: FIXED_TIME,
        kind,
        code,
        value,
    };

    unsafe {
        let ptr = &input_event as *const _ as *const u8;
        let size = mem::size_of_val(&input_event);
        let content = slice::from_raw_parts(ptr, size);
        content
    }
}

impl VirtualDevice {
    pub fn default() -> Result<Self> {
        VirtualDevice::new(
            Duration::from_millis(1),
            50,
        )
    }
    pub fn new(writing_interval: Duration, channel_size: usize) -> Result<Self> {
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

        let (s, r) = bounded(channel_size);

        let mut virtual_device = VirtualDevice {
            writing_interval,
            file,
            def: unsafe { mem::zeroed() },
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

    pub fn write_from_channel_every_ms(mut self) {
        let writing_interval = self.writing_interval;

        let scheduler = thread::spawn(move || {
            loop {
                let start = Instant::now();

                self.write_events_from_channel().unwrap();

                let runtime = start.elapsed();

                if let Some(remaining) = writing_interval.checked_sub(runtime) {
                    sleep(remaining);
                }
            }
        });

        scheduler.join().expect("Scheduler panicked");
    }

    #[inline]
    pub fn write_events_from_channel(&mut self) -> EmptyResult {
        let mut converted = Vec::new();
        self.sender.send(SYN_PARAMS)?;

        for event in self.receiver.try_iter() {
            let content = convert_event_for_writing(event.0, event.1, event.2);
            converted.extend_from_slice(content);
        }

        self.file.write_all(converted.as_slice())?;
        Ok(())
    }

    fn write(&mut self, kind: u16, code: u16, value: i32) -> EmptyResult {
        let content = convert_event_for_writing(kind, code, value);
        self.file.write_all(content)?;
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

    pub fn scroll_x(&mut self, value: Coord) -> EmptyResult {
        self.write(EV_REL, REL_HWHEEL, value)?;
        self.synchronize()
    }

    pub fn scroll_y(&mut self, value: Coord) -> EmptyResult {
        self.write(EV_REL, REL_WHEEL, -value)?;
        self.synchronize()
    }

    pub fn press(&mut self, button: Button) -> EmptyResult {
        self.write(EV_KEY, button, 1)?;
        self.synchronize()
    }

    pub fn release(&mut self, button: Button) -> EmptyResult {
        sleep(SLEEP_BEFORE_RELEASE); // required to preserve typing order

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