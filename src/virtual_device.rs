#![allow(dead_code)]

use crossbeam_channel::{bounded, Receiver, Sender};
use nix::errno::Errno;
use std::ffi::CString;
use std::fs::File;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::thread::{sleep, JoinHandle};
use std::time::{Duration, Instant};
use std::{fs, mem, slice, thread};

use crate::key_codes::KeyCodes;
use crate::utils::GradualMove;
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

const FIXED_TIME: timeval = timeval {
    tv_sec: 0,
    tv_usec: 0,
};
const SYN_PARAMS: EventParams = (KeyCodes::EV_SYN, KeyCodes::SYN_REPORT, 0);

const SLEEP_BEFORE_RELEASE: Duration = Duration::from_millis(5);

#[inline(always)]
fn convert_event_for_writing(
    kind: u16,
    code: u16,
    value: i32,
    input_event: &mut input_event,
) -> Vec<u8> {
    input_event.time = FIXED_TIME;
    input_event.kind = kind;
    input_event.code = code;
    input_event.value = value;

    // let mut input_event = input_event {
    //     time: FIXED_TIME,
    //     kind,
    //     code,
    //     value,
    // };

    unsafe {
        // gettimeofday(&mut input_event.time, ptr::null_mut());

        let ptr = input_event as *const _ as *const u8;
        let size = mem::size_of_val(input_event);
        let content = slice::from_raw_parts(ptr, size);
        content.to_vec()
    }
}

pub enum DeviceDefinitionType {
    Separate,
    MouseOnly,
    KeyboardOnly,
    None,
}

const UINPUT_NOT_LOADED_ERR: &str =
    "'uinput' module probably is not loaded. try: 'sudo modprobe uinput'";

impl VirtualDevice {
    pub fn default() -> Result<Self> {
        Self::default_single_device(DeviceDefinitionType::None)
    }

    fn default_single_device(definition_type: DeviceDefinitionType) -> Result<Self> {
        Self::new(Duration::from_millis(1), 50, definition_type)
    }

    pub fn default_separate() -> Result<(Self, Self)> {
        Ok((
            Self::default_single_device(DeviceDefinitionType::MouseOnly)?,
            Self::default_single_device(DeviceDefinitionType::KeyboardOnly)?,
        ))
    }

    fn new(
        writing_interval: Duration,
        channel_size: usize,
        definition_type: DeviceDefinitionType,
    ) -> Result<Self> {
        let (s, r) = bounded(channel_size);

        let path = Path::new("/dev/uinput");

        #[cfg(feature = "auto-acquire-permissions")]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(path).expect(UINPUT_NOT_LOADED_ERR);
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o660);
        }

        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;

        let file = OpenOptions::new()
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            // .custom_flags(libc::O_WRONLY | libc::O_NDELAY)
            .open(path)?;

        // Mouse:
        // Bus=0003 Vendor=045e Product=07a5 Version=0111
        // Keyboard:
        // Bus=0011 Vendor=0001 Product=0001 Version=ab83

        let mut def: uinput_user_dev = unsafe { mem::zeroed() };
        let device_name: String;

        match definition_type {
            DeviceDefinitionType::Separate => {
                return Err(Box::from("Not implemented"));
            }
            DeviceDefinitionType::MouseOnly => {
                def.id = input_id {
                    bustype: 0x0003,
                    vendor: 0x045e,
                    product: 0x07a5,
                    version: 0x0111,
                };
                device_name = String::from("virtual-mouse");
            }
            DeviceDefinitionType::KeyboardOnly => {
                def.id = input_id {
                    bustype: 0x0011,
                    vendor: 0x0001,
                    product: 0x0001,
                    version: 0xab83,
                };
                device_name = String::from("virtual-keyboard");
            }
            DeviceDefinitionType::None => {
                device_name = String::from("virtual-device");
            }
        }

        let mut virtual_device = VirtualDevice {
            writing_interval,
            file,
            def,
            sender: s,
            receiver: r,
        };

        // let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        // let device_name = format!("virtualdevice-{}", now.as_millis());
        // println!("{}", device_name);

        virtual_device.set_name(device_name.as_str())?;

        match definition_type {
            DeviceDefinitionType::Separate => {
                return Err(Box::from("Not implemented"));
            }
            DeviceDefinitionType::MouseOnly => {
                virtual_device.register_mouse()?;
            }
            DeviceDefinitionType::KeyboardOnly => {
                virtual_device.register_keyboard()?;
            }
            DeviceDefinitionType::None => {
                virtual_device.register_mouse()?;
                virtual_device.register_keyboard()?;
            }
        }

        virtual_device.create()?;

        Ok(virtual_device)
    }

    fn set_name<T: AsRef<str>>(&mut self, value: T) -> EmptyResult {
        let string = CString::new(value.as_ref())?;
        let bytes = string.as_bytes_with_nul();

        if bytes.len() > UINPUT_MAX_NAME_SIZE {
            return Err(Box::from(format!(
                "Virtual device name is longer than maximum allowed size: {}.\nUse shorter name",
                UINPUT_MAX_NAME_SIZE
            )));
        }

        (&mut self.def.name)[..bytes.len()].clone_from_slice(unsafe { mem::transmute(bytes) });

        Ok(())
    }

    fn create(&mut self) -> EmptyResult {
        unsafe {
            let ptr = &self.def as *const _ as *const u8;
            let size = mem::size_of_val(&self.def);
            let as_slice = slice::from_raw_parts(ptr, size);

            self.file.write_all(as_slice)?;

            Errno::result(ui_dev_create(self.file.as_raw_fd()))?;
        }
        Ok(())
    }

    fn register_keyboard(&self) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), KeyCodes::EV_KEY as i32))?;
        }
        for code in 1..255 {
            self.register_key(code)?
        }
        Ok(())
    }

    fn register_mouse(&self) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), KeyCodes::EV_KEY as i32))?;
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), KeyCodes::EV_REL as i32))?;
        }
        for code in [
            KeyCodes::BTN_LEFT,
            KeyCodes::BTN_RIGHT,
            KeyCodes::BTN_MIDDLE,
        ] {
            self.register_key(code)?
        }

        for code in [
            KeyCodes::REL_X,
            KeyCodes::REL_Y,
            KeyCodes::REL_HWHEEL,
            KeyCodes::REL_WHEEL,
        ] {
            self.register_relative(code)?
        }

        Ok(())
    }

    fn register_key(&self, code: u16) -> EmptyResult {
        unsafe {
            // Errno::result(ui_set_evbit(self.file.as_raw_fd(), KeyCodes::EV_KEY as i32))?;

            Errno::result(ui_set_keybit(self.file.as_raw_fd(), code as i32))?;
        }
        Ok(())
    }

    fn register_relative(&self, code: u16) -> EmptyResult {
        unsafe {
            // Errno::result(ui_set_evbit(self.file.as_raw_fd(), KeyCodes::EV_REL as i32))?;

            Errno::result(ui_set_relbit(self.file.as_raw_fd(), code as i32))?;
        }
        Ok(())
    }

    #[inline]
    pub fn send_to_channel(
        kind: u16,
        code: u16,
        value: i32,
        sender: &ChannelSender,
    ) -> EmptyResult {
        sender.send((kind, code, value))?;
        Ok(())
    }

    #[inline]
    pub fn send_press(button: Button, sender: &ChannelSender) -> EmptyResult {
        sender.send((KeyCodes::EV_KEY, button, 1))?;
        sender.send(SYN_PARAMS)?;
        Ok(())
    }

    #[inline]
    pub fn send_release(button: Button, sender: &ChannelSender) -> EmptyResult {
        sender.send((KeyCodes::EV_KEY, button, 0))?;
        Ok(())
    }

    pub fn send_click(button: Button, sender: &ChannelSender) -> EmptyResult {
        VirtualDevice::send_press(button, sender)?;
        VirtualDevice::send_release(button, sender)
    }

    #[inline]
    pub fn send_mouse_move_x(x: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((KeyCodes::EV_REL, KeyCodes::REL_X, x))?;
        Ok(())
    }

    #[inline]
    pub fn send_mouse_move_y(y: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((KeyCodes::EV_REL, KeyCodes::REL_Y, -y))?;
        Ok(())
    }

    #[inline]
    pub fn send_mouse_move(x: Coord, y: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((KeyCodes::EV_REL, KeyCodes::REL_X, x))?;
        sender.send((KeyCodes::EV_REL, KeyCodes::REL_Y, -y))?;
        Ok(())
    }

    #[inline]
    pub fn send_scroll_x(value: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((KeyCodes::EV_REL, KeyCodes::REL_HWHEEL, value))?;
        Ok(())
    }

    #[inline]
    pub fn send_scroll_y(value: Coord, sender: &ChannelSender) -> EmptyResult {
        sender.send((KeyCodes::EV_REL, KeyCodes::REL_WHEEL, value))?;
        Ok(())
    }

    pub fn flush_channel_every_interval(mut self) -> JoinHandle<()> {
        let writing_interval = self.writing_interval;

        thread::spawn(move || loop {
            let start = Instant::now();

            self.write_events_from_channel().unwrap();

            let runtime = start.elapsed();

            if let Some(remaining) = writing_interval.checked_sub(runtime) {
                sleep(remaining);
            }
        })

        // scheduler.join().expect("Scheduler panicked");
    }

    #[inline]
    fn write_events_from_channel(&mut self) -> EmptyResult {
        let mut converted = Vec::new();
        self.sender.send(SYN_PARAMS)?;

        for event in self.receiver.try_iter() {
            // let mut content = convert_event_for_writing(event.0, event.1, event.2);
            // converted.append(&mut content);

            let input_event = input_event {
                time: FIXED_TIME,
                kind: event.0,
                code: event.1,
                value: event.2,
            };

            unsafe {
                // gettimeofday(&mut input_event.time, ptr::null_mut());

                let ptr = &input_event as *const _ as *const u8;
                let size = mem::size_of_val(&input_event);
                let content = slice::from_raw_parts(ptr, size);
                converted.extend_from_slice(content);
            }
        }

        self.file.write_all(converted.as_slice())?;
        Ok(())
    }

    #[inline]
    pub fn write_batch(&mut self, batch: &[EventParams]) -> EmptyResult {
        let mut converted = Vec::new();

        for event in batch {
            let input_event = input_event {
                time: FIXED_TIME,
                kind: event.0,
                code: event.1,
                value: event.2,
            };

            unsafe {
                // gettimeofday(&mut input_event.time, ptr::null_mut());

                let ptr = &input_event as *const _ as *const u8;
                let size = mem::size_of_val(&input_event);
                let content = slice::from_raw_parts(ptr, size);
                converted.extend_from_slice(content);
            }
        }
        self.file.write_all(converted.as_slice())?;
        Ok(())
    }

    #[inline]
    fn write(&mut self, kind: u16, code: u16, value: i32) -> EmptyResult {
        // let content = convert_event_for_writing(kind, code, value);
        // self.file.write_all(content.as_slice())?;

        let input_event = input_event {
            time: FIXED_TIME,
            kind,
            code,
            value,
        };

        unsafe {
            // gettimeofday(&mut input_event.time, ptr::null_mut());

            let ptr = &input_event as *const _ as *const u8;
            let size = mem::size_of_val(&input_event);
            let content = slice::from_raw_parts(ptr, size);
            self.file.write_all(content)?;
        }

        Ok(())
    }

    #[inline(always)]
    pub fn synchronize(&mut self) -> EmptyResult {
        self.write(KeyCodes::EV_SYN, KeyCodes::SYN_REPORT, 0)
    }

    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: Coord) -> EmptyResult {
        self.write(KeyCodes::EV_REL, KeyCodes::REL_X, x)
    }

    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: Coord) -> EmptyResult {
        self.write(KeyCodes::EV_REL, KeyCodes::REL_Y, -y)
    }

    #[inline]
    pub fn move_mouse_raw(&mut self, x: Coord, y: Coord) -> EmptyResult {
        self.write_batch(&[
            (KeyCodes::EV_REL, KeyCodes::REL_X, x),
            (KeyCodes::EV_REL, KeyCodes::REL_Y, -y),
        ])
    }

    #[inline]
    pub fn buffered_move_mouse_x(&mut self, x: Coord) -> Vec<EventParams> {
        vec![(KeyCodes::EV_REL, KeyCodes::REL_X, x), SYN_PARAMS]
    }

    #[inline]
    pub fn buffered_move_mouse_y(&mut self, y: Coord) -> Vec<EventParams> {
        vec![(KeyCodes::EV_REL, KeyCodes::REL_Y, -y), SYN_PARAMS]
    }

    #[inline]
    pub fn buffered_move_mouse(&mut self, x: Coord, y: Coord) -> Vec<EventParams> {
        vec![
            (KeyCodes::EV_REL, KeyCodes::REL_X, x),
            (KeyCodes::EV_REL, KeyCodes::REL_Y, -y),
            SYN_PARAMS,
        ]
    }

    #[inline]
    pub fn gradual_move_mouse_raw(&mut self, x: Coord, y: Coord) -> Result<()> {
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            self.move_mouse_raw(gradual_move.x_direction, gradual_move.y_direction)?;
        }
        for _ in 0..gradual_move.move_only_x {
            self.move_mouse_raw_x(gradual_move.x_direction)?;
        }
        for _ in 0..gradual_move.move_only_y {
            self.move_mouse_raw_y(gradual_move.y_direction)?;
        }
        self.synchronize()?;

        Ok(())
    }

    #[inline]
    pub fn gradual_move_mouse(&mut self, x: Coord, y: Coord) -> Result<()> {
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            self.move_mouse(gradual_move.x_direction, gradual_move.y_direction)?;
        }
        for _ in 0..gradual_move.move_only_x {
            self.move_mouse_x(gradual_move.x_direction)?;
        }
        for _ in 0..gradual_move.move_only_y {
            self.move_mouse_y(gradual_move.y_direction)?;
        }

        Ok(())
    }

    #[inline]
    pub fn smooth_move_mouse(&mut self, x: Coord, y: Coord) -> Result<()> {
        self.gradual_move_mouse_raw(x, y)
    }

    #[inline]
    pub fn buffered_gradual_move_mouse(&mut self, x: Coord, y: Coord) -> Vec<EventParams> {
        let mut write_buffer: Vec<EventParams> = vec![];
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            write_buffer.extend(
                self.buffered_move_mouse(gradual_move.x_direction, gradual_move.y_direction),
            );
        }
        for _ in 0..gradual_move.move_only_x {
            write_buffer.extend(self.buffered_move_mouse_x(gradual_move.x_direction));
        }
        for _ in 0..gradual_move.move_only_y {
            write_buffer.extend(self.buffered_move_mouse_y(gradual_move.y_direction));
        }

        write_buffer
    }

    #[inline]
    pub fn move_mouse_x(&mut self, x: Coord) -> EmptyResult {
        self.write_batch(&[(KeyCodes::EV_REL, KeyCodes::REL_X, x), SYN_PARAMS])
    }

    #[inline]
    pub fn move_mouse_y(&mut self, y: Coord) -> EmptyResult {
        self.write_batch(&[(KeyCodes::EV_REL, KeyCodes::REL_Y, -y), SYN_PARAMS])
    }

    #[inline]
    pub fn move_mouse(&mut self, x: Coord, y: Coord) -> EmptyResult {
        self.write_batch(&[
            (KeyCodes::EV_REL, KeyCodes::REL_X, x),
            (KeyCodes::EV_REL, KeyCodes::REL_Y, -y),
            SYN_PARAMS,
        ])
    }

    // #[inline]
    // pub fn move_mouse_with_options(&mut self, x: Coord, y: Coord, buffered: bool, gradual_move: bool, raw_operations:bool) -> EmptyResult {
    //     let (mouse_x, mouse_y, mouse) = match buffered {
    //         true => {
    //             (
    //                 Self::buffered_move_mouse_x,
    //                 Self::buffered_move_mouse_y,
    //                 Self::buffered_move_mouse,
    //             )
    //         }
    //         false => {
    //             match raw_operations {
    //                 true => {
    //                     (
    //                         Self::move_mouse_raw_x,
    //                         Self::move_mouse_raw_y,
    //                         Self::move_mouse_raw,
    //                     )
    //                 }
    //                 false => {
    //                     (
    //                         Self::move_mouse_x,
    //                         Self::move_mouse_y,
    //                         Self::move_mouse,
    //                     )
    //                 }
    //             }
    //         }
    //     };
    //     match gradual_move {
    //         true => {
    //
    //         }
    //         false => {}
    //     }
    // }

    #[inline]
    pub fn scroll_raw_x(&mut self, value: Coord) -> EmptyResult {
        self.write(KeyCodes::EV_REL, KeyCodes::REL_HWHEEL, value)
    }

    #[inline]
    pub fn scroll_raw_y(&mut self, value: Coord) -> EmptyResult {
        self.write(KeyCodes::EV_REL, KeyCodes::REL_WHEEL, value)
    }

    #[inline]
    pub fn buffered_scroll_x(&mut self, value: Coord) -> Vec<EventParams> {
        vec![(KeyCodes::EV_REL, KeyCodes::REL_HWHEEL, value), SYN_PARAMS]
    }

    #[inline]
    pub fn buffered_scroll_y(&mut self, value: Coord) -> Vec<EventParams> {
        vec![(KeyCodes::EV_REL, KeyCodes::REL_WHEEL, value), SYN_PARAMS]
    }

    #[inline]
    pub fn scroll_x(&mut self, value: Coord) -> EmptyResult {
        self.write_batch(&[(KeyCodes::EV_REL, KeyCodes::REL_HWHEEL, value), SYN_PARAMS])
    }

    #[inline]
    pub fn gradual_scroll_raw(&mut self, x: Coord, y: Coord) -> Result<()> {
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            self.scroll_raw_x(gradual_move.x_direction)?;
            self.scroll_raw_y(gradual_move.y_direction)?;
        }
        for _ in 0..gradual_move.move_only_x {
            self.scroll_raw_x(gradual_move.x_direction)?;
        }
        for _ in 0..gradual_move.move_only_y {
            self.scroll_raw_y(gradual_move.y_direction)?;
        }
        self.synchronize()?;

        Ok(())
    }

    #[inline]
    pub fn gradual_scroll(&mut self, x: Coord, y: Coord) -> Result<()> {
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            self.scroll_x(gradual_move.x_direction)?;
            self.scroll_y(gradual_move.y_direction)?;
        }
        for _ in 0..gradual_move.move_only_x {
            self.scroll_x(gradual_move.x_direction)?;
        }
        for _ in 0..gradual_move.move_only_y {
            self.scroll_y(gradual_move.y_direction)?;
        }

        Ok(())
    }

    #[inline]
    pub fn smooth_scroll(&mut self, x: Coord, y: Coord) -> Result<()> {
        self.gradual_scroll_raw(x, y)
    }

    #[inline]
    pub fn buffered_gradual_scroll(&mut self, x: Coord, y: Coord) -> Vec<EventParams> {
        let mut write_buffer: Vec<EventParams> = vec![];
        let gradual_move = GradualMove::calculate(x, y);

        for _ in 0..gradual_move.both_move {
            write_buffer.extend(self.buffered_scroll_x(gradual_move.x_direction));
            write_buffer.extend(self.buffered_scroll_y(gradual_move.y_direction));
        }
        for _ in 0..gradual_move.move_only_x {
            write_buffer.extend(self.buffered_scroll_x(gradual_move.x_direction));
        }
        for _ in 0..gradual_move.move_only_y {
            write_buffer.extend(self.buffered_scroll_y(gradual_move.y_direction));
        }

        write_buffer
    }

    #[inline]
    pub fn scroll_y(&mut self, value: Coord) -> EmptyResult {
        self.write_batch(&[(KeyCodes::EV_REL, KeyCodes::REL_WHEEL, value), SYN_PARAMS])
    }

    #[inline]
    pub fn buffered_press(&mut self, button: Button) -> Vec<EventParams> {
        vec![(KeyCodes::EV_KEY, button, 1), SYN_PARAMS]
    }

    #[inline]
    pub fn buffered_release(&mut self, button: Button) -> Vec<EventParams> {
        vec![(KeyCodes::EV_KEY, button, 0), SYN_PARAMS]
    }

    #[inline]
    pub fn press(&mut self, button: Button) -> EmptyResult {
        self.write_batch(&[(KeyCodes::EV_KEY, button, 1), SYN_PARAMS])
    }

    #[inline]
    pub fn release(&mut self, button: Button) -> EmptyResult {
        self.write_batch(&[(KeyCodes::EV_KEY, button, 0), SYN_PARAMS])
    }

    pub fn click(&mut self, button: Button) -> EmptyResult {
        self.press(button)?;
        sleep(SLEEP_BEFORE_RELEASE); // required to preserve typing order
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

