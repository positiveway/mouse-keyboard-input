#[macro_use]
extern crate ioctl_sys as ioctl;
extern crate libc;
extern crate nix;

use libc::timeval;
use std::mem;

macro_rules! uin {
	(write $name:ident with $ioty:expr, $nr:expr; $ty:ty) => (
		pub unsafe fn $name(fd: i32, val: $ty) -> i32 {
			ioctl::ioctl(fd, iow!($ioty, $nr, mem::size_of::<$ty>()) as u64, val)
		}
	);
}


pub mod key_codes;

pub use crate::key_codes::*;

mod virtual_device;

pub use virtual_device::VirtualDevice;

mod error;

pub use error::Error;

pub const UINPUT_MAX_NAME_SIZE: i32 = 80;


#[derive(Clone, Copy)]
#[repr(C)]
pub struct input_event {
    pub time: timeval,
    pub kind: u16,
    pub code: u16,
    pub value: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct input_id {
    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

#[derive(Debug)]
#[repr(C)]
pub struct uinput_user_dev {
    pub name: [i8; UINPUT_MAX_NAME_SIZE as usize],
    pub id: input_id,

    pub ff_effects_max: u32,
    pub absmax: [i32; ABS_CNT as usize],
    pub absmin: [i32; ABS_CNT as usize],
    pub absfuzz: [i32; ABS_CNT as usize],
    pub absflat: [i32; ABS_CNT as usize],
}

//#[repr(C)]
//pub struct uinput_ff_upload {
//	pub request_id: u32,
//	pub retval:     i32,
//	pub effect:     ff_effect,
//	pub old:        ff_effect,
//}
//
//#[repr(C)]
//pub struct uinput_ff_erase {
//	pub request_id: u32,
//	pub retval:     i32,
//	pub effect_id:  u32,
//}

ioctl!(none ui_dev_create with b'U', 1);
ioctl!(none ui_dev_destroy with b'U', 2);

uin!(write ui_set_evbit   with b'U', 100; i32);
uin!(write ui_set_keybit  with b'U', 101; i32);
uin!(write ui_set_relbit  with b'U', 102; i32);
uin!(write ui_set_absbit  with b'U', 103; i32);
uin!(write ui_set_mscbit  with b'U', 104; i32);
uin!(write ui_set_ledbit  with b'U', 105; i32);
uin!(write ui_set_sndbit  with b'U', 106; i32);
uin!(write ui_set_ffbit   with b'U', 107; i32);
uin!(write ui_set_phys    with b'U', 108; *const i8);
uin!(write ui_set_swbit   with b'U', 109; i32);
uin!(write ui_set_propbit with b'U', 110; i32);

//ioctl!(readwrite ui_begin_ff_upload with b'U', 200, uinput_ff_upload);
//ioctl!(readwrite ui_end_ff_upload with b'U', 201, uinput_ff_upload);

//ioctl!(readwrite ui_begin_ff_erase with b'U', 200, uinput_ff_erase);
//ioctl!(readwrite ui_end_ff_erase with b'U', 201, uinput_ff_erase);

ioctl!(read ui_get_version with b'U', 45; u32);
