use crate::utils::GradualMove;
use crate::*;
use crate::virtual_device::{UINPUT_NOT_LOADED_ERR, SLEEP_BEFORE_RELEASE, FIXED_TIME};
use crossbeam_channel::{Receiver, Sender, bounded};
use io_uring::{opcode, squeue, types, IoUring};
use nix::errno::Errno;
use std::collections::VecDeque;
use std::ffi::CString;
use std::fs::File;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::thread::{self, JoinHandle, sleep};
use std::time::{Duration, Instant};
use std::{fs, mem, slice};

const IO_URING_ENTRIES: u32 = 64;
const IO_URING_BUFFERS: usize = 64;
const IO_URING_BUFFER_SIZE: usize = 4096;

pub struct VirtualDeviceUring {
    writing_interval: Duration,
    file: File,
    def: uinput_user_dev,
    pub sender: ChannelSender,
    receiver: Receiver<EventParams>,
    ring: IoUring,
    buffers: Vec<Vec<u8>>,
    free_buffers: VecDeque<usize>,
    outstanding: u32,
}

impl VirtualDeviceUring {
    pub fn new(definition_type: DeviceDefinitionType) -> Result<Self> {
        let (s, r) = bounded(50);
        let writing_interval = Duration::from_millis(1);

        let path = Path::new("/dev/uinput");

        #[cfg(feature = "auto-acquire-permissions")]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(path).expect(UINPUT_NOT_LOADED_ERR);
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o660);
            let _ = fs::set_permissions(path, permissions);
        }

        use std::fs::OpenOptions;
        use std::os::unix::fs::OpenOptionsExt;

        // CRITICAL: O_NONBLOCK is required for io_uring to process writes inline.
        // Without it, io_uring punts writes to a worker thread, breaking event
        // ordering, adding latency, and causing evdev buffer overflows.
        let file = OpenOptions::new()
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)?;

        let ring = IoUring::new(IO_URING_ENTRIES)?;

        let fds = [file.as_raw_fd()];
        ring.submitter().register_files(&fds)?;

        let buffers = (0..IO_URING_BUFFERS)
            .map(|_| Vec::with_capacity(IO_URING_BUFFER_SIZE))
            .collect();
        let free_buffers = (0..IO_URING_BUFFERS).collect();

        let mut def: uinput_user_dev = unsafe { mem::zeroed() };
        let mut device_name: String;

        match definition_type {
            DeviceDefinitionType::Separate => return Err(Box::from("Not implemented")),
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

        let mut virtual_device = VirtualDeviceUring {
            writing_interval,
            file,
            def,
            sender: s,
            receiver: r,
            ring,
            buffers,
            free_buffers,
            outstanding: 0,
        };

        virtual_device.set_name(device_name.as_str())?;

        match definition_type {
            DeviceDefinitionType::Separate => return Err(Box::from("Not implemented")),
            DeviceDefinitionType::MouseOnly => virtual_device.register_mouse()?,
            DeviceDefinitionType::KeyboardOnly => virtual_device.register_keyboard()?,
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
            return Err(Box::from(format!("Name too long")));
        }
        let signed_bytes: &[i8] =
            unsafe { slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len()) };
        self.def.name[..bytes.len()].clone_from_slice(signed_bytes);
        Ok(())
    }

    fn create(&mut self) -> EmptyResult {
        unsafe {
            let ptr = &self.def as *const _ as *const u8;
            let size = mem::size_of_val(&self.def);
            let as_slice = slice::from_raw_parts(ptr, size);
            self.write_all(as_slice)?;
            Errno::result(ui_dev_create(self.file.as_raw_fd()))?;
        }
        Ok(())
    }

    fn register_keyboard(&self) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), EV_KEY as i32))?;
        }
        for code in 1..255 {
            self.register_key(code)?;
        }
        Ok(())
    }

    fn register_mouse(&self) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), EV_KEY as i32))?;
            Errno::result(ui_set_evbit(self.file.as_raw_fd(), EV_REL as i32))?;
        }
        for code in [BTN_LEFT, BTN_RIGHT, BTN_MIDDLE] {
            self.register_key(code)?;
        }
        for code in [REL_X, REL_Y, REL_HWHEEL, REL_WHEEL] {
            self.register_relative(code)?;
        }
        Ok(())
    }

    fn register_key(&self, code: u16) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_keybit(self.file.as_raw_fd(), code as i32))?;
        }
        Ok(())
    }

    fn register_relative(&self, code: u16) -> EmptyResult {
        unsafe {
            Errno::result(ui_set_relbit(self.file.as_raw_fd(), code as i32))?;
        }
        Ok(())
    }

    fn reap_completions(&mut self) -> EmptyResult {
        let mut first_error: Option<i32> = None;
        for cqe in self.ring.completion() {
            if self.outstanding > 0 {
                self.outstanding -= 1;
            }
            self.free_buffers.push_back(cqe.user_data() as usize);
            let res = cqe.result();
            if res < 0 && first_error.is_none() {
                first_error = Some(res);
            }
        }
        if let Some(res) = first_error {
            return Err(Box::from(std::io::Error::from_raw_os_error(-res)));
        }
        Ok(())
    }

    fn write_all(&mut self, buf: &[u8]) -> EmptyResult {
        if buf.is_empty() {
            return Ok(());
        }

        self.reap_completions()?;

        while self.free_buffers.is_empty() {
            self.ring
                .submit_and_wait(1)
                .map_err(|e| {
                    Box::<dyn std::error::Error>::from(format!(
                        "io_uring submit_and_wait (buffer wait) failed: {}",
                        e
                    ))
                })?;
            self.reap_completions()?;
        }

        let buf_idx = self
            .free_buffers
            .pop_front()
            .expect("free buffer guaranteed by loop above");

        let (ptr, len) = {
            let buffer = &mut self.buffers[buf_idx];
            if buffer.len() < buf.len() {
                buffer.resize(buf.len(), 0);
            }
            buffer[..buf.len()].copy_from_slice(buf);
            (buffer.as_ptr(), buf.len() as u32)
        };

        let entry = opcode::Write::new(types::Fd(0), ptr, len)
            .build()
            .flags(squeue::Flags::FIXED_FILE)
            .user_data(buf_idx as u64);

        unsafe {
            self.ring.submission().push(&entry).map_err(|e| {
                Box::<dyn std::error::Error>::from(format!(
                    "io_uring submission push failed: {:?}",
                    e
                ))
            })?;
        }
        self.outstanding += 1;

        self.ring
            .submit_and_wait(self.outstanding as usize)
            .map_err(|e| {
                Box::<dyn std::error::Error>::from(format!(
                    "io_uring submit_and_wait failed: {}",
                    e
                ))
            })?;

        self.reap_completions()?;

        Ok(())
    }

    pub fn flush_channel_every_interval(mut self) -> JoinHandle<()> {
        let writing_interval = self.writing_interval;
        thread::spawn(move || {
            loop {
                let start = Instant::now();
                self.write_events_from_channel().unwrap();
                let runtime = start.elapsed();
                if let Some(remaining) = writing_interval.checked_sub(runtime) {
                    sleep(remaining);
                }
            }
        })
    }

    #[inline]
    fn write_events_from_channel(&mut self) -> EmptyResult {
        self.sender.send(SYN_PARAMS)?;
        let mut batch = Vec::with_capacity(64);
        for event in self.receiver.try_iter() {
            batch.push(event);
        }
        self.write_batch(&batch)
    }

    #[inline]
    pub fn write_batch(&mut self, batch: &[EventParams]) -> EmptyResult {
        if batch.is_empty() {
            return Ok(());
        }
        let mut converted = Vec::with_capacity(batch.len() * mem::size_of::<input_event>());
        for event in batch {
            let input_event = input_event {
                time: FIXED_TIME,
                kind: event.0,
                code: event.1,
                value: event.2,
            };
            unsafe {
                let ptr = &input_event as *const _ as *const u8;
                let size = mem::size_of_val(&input_event);
                let content = slice::from_raw_parts(ptr, size);
                converted.extend_from_slice(content);
            }
        }
        self.write_all(converted.as_slice())
    }

    #[inline]
    fn write(&mut self, kind: u16, code: u16, value: i32) -> EmptyResult {
        self.write_batch(&[(kind, code, value)])
    }

    #[inline(always)]
    pub fn synchronize(&mut self) -> EmptyResult {
        self.write(EV_SYN, SYN_REPORT, 0)
    }

    #[inline]
    pub fn move_mouse_raw_x(&mut self, x: Coord) -> EmptyResult {
        self.write(EV_REL, REL_X, x)
    }
    #[inline]
    pub fn move_mouse_raw_y(&mut self, y: Coord) -> EmptyResult {
        self.write(EV_REL, REL_Y, -y)
    }
    #[inline]
    pub fn move_mouse_raw(&mut self, x: Coord, y: Coord) -> EmptyResult {
        self.write_batch(&[(EV_REL, REL_X, x), (EV_REL, REL_Y, -y)])
    }

    #[inline]
    pub fn buffered_move_mouse_x(&mut self, x: Coord) -> Vec<EventParams> {
        vec![(EV_REL, REL_X, x), SYN_PARAMS]
    }
    #[inline]
    pub fn buffered_move_mouse_y(&mut self, y: Coord) -> Vec<EventParams> {
        vec![(EV_REL, REL_Y, -y), SYN_PARAMS]
    }
    #[inline]
    pub fn buffered_move_mouse(&mut self, x: Coord, y: Coord) -> Vec<EventParams> {
        vec![(EV_REL, REL_X, x), (EV_REL, REL_Y, -y), SYN_PARAMS]
    }

    #[inline]
    pub fn gradual_move_mouse_raw(&mut self, x: Coord, y: Coord) -> Result<()> {
        let g = GradualMove::calculate(x, y);
        for _ in 0..g.both_move {
            self.move_mouse_raw(g.x_direction, g.y_direction)?;
        }
        for _ in 0..g.move_only_x {
            self.move_mouse_raw_x(g.x_direction)?;
        }
        for _ in 0..g.move_only_y {
            self.move_mouse_raw_y(g.y_direction)?;
        }
        self.synchronize()?;
        Ok(())
    }

    #[inline]
    pub fn gradual_move_mouse(&mut self, x: Coord, y: Coord) -> Result<()> {
        let g = GradualMove::calculate(x, y);
        for _ in 0..g.both_move {
            self.move_mouse(g.x_direction, g.y_direction)?;
        }
        for _ in 0..g.move_only_x {
            self.move_mouse_x(g.x_direction)?;
        }
        for _ in 0..g.move_only_y {
            self.move_mouse_y(g.y_direction)?;
        }
        Ok(())
    }

    #[inline]
    pub fn smooth_move_mouse(&mut self, x: Coord, y: Coord) -> Result<()> {
        self.gradual_move_mouse_raw(x, y)
    }

    #[inline]
    pub fn buffered_gradual_move_mouse(&mut self, x: Coord, y: Coord) -> Vec<EventParams> {
        let mut wb = vec![];
        let g = GradualMove::calculate(x, y);
        for _ in 0..g.both_move {
            wb.extend(self.buffered_move_mouse(g.x_direction, g.y_direction));
        }
        for _ in 0..g.move_only_x {
            wb.extend(self.buffered_move_mouse_x(g.x_direction));
        }
        for _ in 0..g.move_only_y {
            wb.extend(self.buffered_move_mouse_y(g.y_direction));
        }
        wb
    }

    #[inline]
    pub fn move_mouse_x(&mut self, x: Coord) -> EmptyResult {
        self.write_batch(&[(EV_REL, REL_X, x), SYN_PARAMS])
    }
    #[inline]
    pub fn move_mouse_y(&mut self, y: Coord) -> EmptyResult {
        self.write_batch(&[(EV_REL, REL_Y, -y), SYN_PARAMS])
    }
    #[inline]
    pub fn move_mouse(&mut self, x: Coord, y: Coord) -> EmptyResult {
        self.write_batch(&[(EV_REL, REL_X, x), (EV_REL, REL_Y, -y), SYN_PARAMS])
    }

    #[inline]
    pub fn scroll_raw_x(&mut self, value: Coord) -> EmptyResult {
        self.write(EV_REL, REL_HWHEEL, value)
    }
    #[inline]
    pub fn scroll_raw_y(&mut self, value: Coord) -> EmptyResult {
        self.write(EV_REL, REL_WHEEL, value)
    }
    #[inline]
    pub fn buffered_scroll_x(&mut self, value: Coord) -> Vec<EventParams> {
        vec![(EV_REL, REL_HWHEEL, value), SYN_PARAMS]
    }
    #[inline]
    pub fn buffered_scroll_y(&mut self, value: Coord) -> Vec<EventParams> {
        vec![(EV_REL, REL_WHEEL, value), SYN_PARAMS]
    }
    #[inline]
    pub fn scroll_x(&mut self, value: Coord) -> EmptyResult {
        self.write_batch(&[(EV_REL, REL_HWHEEL, value), SYN_PARAMS])
    }

    #[inline]
    pub fn gradual_scroll_raw(&mut self, x: Coord, y: Coord) -> Result<()> {
        let g = GradualMove::calculate(x, y);
        for _ in 0..g.both_move {
            self.scroll_raw_x(g.x_direction)?;
            self.scroll_raw_y(g.y_direction)?;
        }
        for _ in 0..g.move_only_x {
            self.scroll_raw_x(g.x_direction)?;
        }
        for _ in 0..g.move_only_y {
            self.scroll_raw_y(g.y_direction)?;
        }
        self.synchronize()?;
        Ok(())
    }

    #[inline]
    pub fn gradual_scroll(&mut self, x: Coord, y: Coord) -> Result<()> {
        let g = GradualMove::calculate(x, y);
        for _ in 0..g.both_move {
            self.scroll_x(g.x_direction)?;
            self.scroll_y(g.y_direction)?;
        }
        for _ in 0..g.move_only_x {
            self.scroll_x(g.x_direction)?;
        }
        for _ in 0..g.move_only_y {
            self.scroll_y(g.y_direction)?;
        }
        Ok(())
    }

    #[inline]
    pub fn smooth_scroll(&mut self, x: Coord, y: Coord) -> Result<()> {
        self.gradual_scroll_raw(x, y)
    }

    #[inline]
    pub fn buffered_gradual_scroll(&mut self, x: Coord, y: Coord) -> Vec<EventParams> {
        let mut wb = vec![];
        let g = GradualMove::calculate(x, y);
        for _ in 0..g.both_move {
            wb.extend(self.buffered_scroll_x(g.x_direction));
            wb.extend(self.buffered_scroll_y(g.y_direction));
        }
        for _ in 0..g.move_only_x {
            wb.extend(self.buffered_scroll_x(g.x_direction));
        }
        for _ in 0..g.move_only_y {
            wb.extend(self.buffered_scroll_y(g.y_direction));
        }
        wb
    }

    #[inline]
    pub fn scroll_y(&mut self, value: Coord) -> EmptyResult {
        self.write_batch(&[(EV_REL, REL_WHEEL, value), SYN_PARAMS])
    }

    #[inline]
    pub fn buffered_press(&mut self, button: Button) -> Vec<EventParams> {
        vec![(EV_KEY, button, 1), SYN_PARAMS]
    }
    #[inline]
    pub fn buffered_release(&mut self, button: Button) -> Vec<EventParams> {
        vec![(EV_KEY, button, 0), SYN_PARAMS]
    }
    #[inline]
    pub fn press(&mut self, button: Button) -> EmptyResult {
        self.write_batch(&[(EV_KEY, button, 1), SYN_PARAMS])
    }
    #[inline]
    pub fn release(&mut self, button: Button) -> EmptyResult {
        self.write_batch(&[(EV_KEY, button, 0), SYN_PARAMS])
    }

    pub fn click(&mut self, button: Button) -> EmptyResult {
        self.press(button)?;
        sleep(SLEEP_BEFORE_RELEASE);
        self.release(button)
    }
}

impl Drop for VirtualDeviceUring {
    fn drop(&mut self) {
        let _ = self.ring.submit();
        while self.outstanding > 0 {
            if self.ring.submit_and_wait(1).is_err() {
                let _ = self.reap_completions();
                break;
            }
            let _ = self.reap_completions();
        }
        let _ = self.reap_completions();
        unsafe {
            ui_dev_destroy(self.file.as_raw_fd());
        }
    }
}