use crate::backend_io_uring::VirtualDeviceUring;
use crate::backend_fs::VirtualDeviceFs;
use crate::*;
use crossbeam_channel::Sender;
use std::thread::JoinHandle;
use std::time::Duration;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type EmptyResult = Result<()>;

pub type Button = u16;
pub type Coord = i32;

pub type EventParams = (u16, u16, i32);
pub type ChannelSender = Sender<EventParams>;

pub const SYN_PARAMS: EventParams = (EV_SYN, SYN_REPORT, 0);

pub(crate) const UINPUT_NOT_LOADED_ERR: &str =
    "'uinput' module probably is not loaded. try: 'sudo modprobe uinput'";
pub(crate) const SLEEP_BEFORE_RELEASE: Duration = Duration::from_millis(5);
pub(crate) const FIXED_TIME: libc::timeval = libc::timeval { tv_sec: 0, tv_usec: 0 };

#[derive(Clone)]
pub enum DeviceDefinitionType {
    Separate,
    MouseOnly,
    KeyboardOnly,
    None,
}

#[derive(Clone)]
pub enum BackendType {
    Uring,
    Fs,
}

pub enum VirtualDevice {
    Uring(VirtualDeviceUring),
    Fs(VirtualDeviceFs),
}

// Macro must be defined outside of impl blocks in Rust 2024
macro_rules! delegate {
    (self $method:ident) => {
        #[inline]
        pub fn $method(self) -> JoinHandle<()> {
            match self {
                Self::Uring(d) => d.$method(),
                Self::Fs(d) => d.$method(),
            }
        }
    };
    (ret $ret:ty, $method:ident) => {
        #[inline]
        pub fn $method(&mut self) -> $ret {
            match self {
                Self::Uring(d) => d.$method(),
                Self::Fs(d) => d.$method(),
            }
        }
    };
    (ret $ret:ty, $method:ident, $($arg:ident: $ty:ty),*) => {
        #[inline]
        pub fn $method(&mut self, $($arg: $ty),*) -> $ret {
            match self {
                Self::Uring(d) => d.$method($($arg),*),
                Self::Fs(d) => d.$method($($arg),*),
            }
        }
    };
}

impl VirtualDevice {
    pub fn new(backend_type: BackendType, definition_type: DeviceDefinitionType) -> Result<Self> {
        match backend_type {
            BackendType::Uring => Ok(Self::Uring(VirtualDeviceUring::new(definition_type)?)),
            BackendType::Fs => Ok(Self::Fs(VirtualDeviceFs::new(definition_type)?)),
        }
    }

    pub fn default(backend_type: BackendType) -> Result<Self> {
        Self::new(backend_type, DeviceDefinitionType::None)
    }

    pub fn default_separate(backend_type: BackendType) -> Result<(Self, Self)> {
        Ok((
            Self::new(backend_type.clone(), DeviceDefinitionType::MouseOnly)?,
            Self::new(backend_type, DeviceDefinitionType::KeyboardOnly)?,
        ))
    }

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

    pub fn send_click(button: Button, sender: &ChannelSender) -> EmptyResult {
        VirtualDevice::send_press(button, sender)?;
        VirtualDevice::send_release(button, sender)
    }

    #[inline]
    pub fn send_mouse_move_x(x: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((EV_REL, REL_X, x))?;
        Ok(())
    }

    #[inline]
    pub fn send_mouse_move_y(y: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((EV_REL, REL_Y, -y))?;
        Ok(())
    }

    #[inline]
    pub fn send_mouse_move(x: Coord, y: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((EV_REL, REL_X, x))?;
        sender.send((EV_REL, REL_Y, -y))?;
        Ok(())
    }

    #[inline]
    pub fn send_scroll_x(value: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((EV_REL, REL_HWHEEL, value))?;
        Ok(())
    }

    #[inline]
    pub fn send_scroll_y(value: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((EV_REL, REL_WHEEL, value))?;
        Ok(())
    }

    delegate!(self flush_channel_every_interval);
    delegate!(ret EmptyResult, write_batch, batch: &[EventParams]);
    delegate!(ret EmptyResult, synchronize);
    delegate!(ret EmptyResult, move_mouse_raw_x, x: Coord);
    delegate!(ret EmptyResult, move_mouse_raw_y, y: Coord);
    delegate!(ret EmptyResult, move_mouse_raw, x: Coord, y: Coord);

    delegate!(ret Vec<EventParams>, buffered_move_mouse_x, x: Coord);
    delegate!(ret Vec<EventParams>, buffered_move_mouse_y, y: Coord);
    delegate!(ret Vec<EventParams>, buffered_move_mouse, x: Coord, y: Coord);

    delegate!(ret Result<()>, gradual_move_mouse_raw, x: Coord, y: Coord);
    delegate!(ret Result<()>, gradual_move_mouse, x: Coord, y: Coord);
    delegate!(ret Result<()>, smooth_move_mouse, x: Coord, y: Coord);
    delegate!(ret Vec<EventParams>, buffered_gradual_move_mouse, x: Coord, y: Coord);

    delegate!(ret EmptyResult, move_mouse_x, x: Coord);
    delegate!(ret EmptyResult, move_mouse_y, y: Coord);
    delegate!(ret EmptyResult, move_mouse, x: Coord, y: Coord);

    delegate!(ret EmptyResult, scroll_raw_x, value: Coord);
    delegate!(ret EmptyResult, scroll_raw_y, value: Coord);
    delegate!(ret Vec<EventParams>, buffered_scroll_x, value: Coord);
    delegate!(ret Vec<EventParams>, buffered_scroll_y, value: Coord);
    delegate!(ret EmptyResult, scroll_x, value: Coord);

    delegate!(ret Result<()>, gradual_scroll_raw, x: Coord, y: Coord);
    delegate!(ret Result<()>, gradual_scroll, x: Coord, y: Coord);
    delegate!(ret Result<()>, smooth_scroll, x: Coord, y: Coord);
    delegate!(ret Vec<EventParams>, buffered_gradual_scroll, x: Coord, y: Coord);
    delegate!(ret EmptyResult, scroll_y, value: Coord);

    delegate!(ret Vec<EventParams>, buffered_press, button: Button);
    delegate!(ret Vec<EventParams>, buffered_release, button: Button);
    delegate!(ret EmptyResult, press, button: Button);
    delegate!(ret EmptyResult, release, button: Button);
    delegate!(ret EmptyResult, click, button: Button);
}