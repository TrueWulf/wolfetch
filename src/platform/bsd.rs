use crate::model::{Config, Field, Info};

unsafe extern "C" {
    fn clock_gettime(clock: i32, time: *mut Timespec) -> i32;
    fn gethostname(name: *mut u8, length: usize) -> i32;
    fn isatty(fd: i32) -> i32;
    fn open(path: *const u8, flags: i32) -> i32;
    fn close(fd: i32) -> i32;
    fn read(fd: i32, buffer: *mut u8, length: usize) -> isize;
    fn uname(name: *mut Utsname) -> i32;
    #[link_name = "write"]
    fn bsd_write(fd: i32, buffer: *const u8, length: usize) -> isize;
}

#[repr(C)]
struct Timespec {
    seconds: i64,
    nanos: i64,
}

#[repr(C)]
struct Utsname {
    sysname: [u8; 256],
    nodename: [u8; 256],
    release: [u8; 256],
    version: [u8; 256],
    machine: [u8; 256],
}

pub fn now() -> u64 {
    let mut time = Timespec {
        seconds: 0,
        nanos: 0,
    };
    unsafe { clock_gettime(1, &mut time) };
    time.seconds as u64 * 1_000_000 + time.nanos as u64 / 1_000
}

pub unsafe fn isatty_stdout() -> bool {
    unsafe { isatty(1) != 0 }
}

pub unsafe fn write(fd: i32, buffer: *const u8, length: usize) -> isize {
    unsafe { bsd_write(fd, buffer, length) }
}

pub fn os_label() -> &'static [u8] {
    b"OS"
}

pub fn read_file(path: &[u8], buffer: &mut [u8]) -> usize {
    let fd = unsafe { open(path.as_ptr(), 0) };
    if fd < 0 {
        return 0;
    }
    let length = unsafe { read(fd, buffer.as_mut_ptr(), buffer.len()) };
    unsafe { close(fd) };
    if length > 0 { length as usize } else { 0 }
}

pub fn env_copy(name: &[u8], output: &mut [u8]) -> usize {
    copy_cstr(unsafe { getenv(name.as_ptr()) }, output)
}

pub fn copy_cstr(pointer: *const u8, output: &mut [u8]) -> usize {
    if pointer.is_null() {
        return 0;
    }
    let mut index = 0;
    while index + 1 < output.len() {
        let byte = unsafe { *pointer.add(index) };
        output[index] = byte;
        if byte == 0 {
            return index + 1;
        }
        index += 1;
    }
    0
}

pub fn collect(config: &Config, start: u64) -> Info {
    let mut values = [Field::new(); 16];
    let mut system = Utsname {
        sysname: [0; 256],
        nodename: [0; 256],
        release: [0; 256],
        version: [0; 256],
        machine: [0; 256],
    };
    let has_uname = unsafe { uname(&mut system) } == 0;
    if config.show & 1 != 0 {
        copy_text(&system.sysname, &mut values[0], has_uname);
    }
    if config.show & (1 << 1) != 0 {
        copy_text(&system.release, &mut values[1], has_uname);
    }
    if config.show & (1 << 2) != 0 {
        env_text(b"XDG_CURRENT_DESKTOP\0", &mut values[2]);
    }
    if config.show & (1 << 3) != 0 {
        env_text(b"TERM\0", &mut values[3]);
    }
    if config.show & (1 << 4) != 0 {
        env_text(b"SHELL\0", &mut values[4]);
        values[4].slash_tail();
    }
    if config.show & (1 << 5) != 0 {
        copy_text(&system.machine, &mut values[5], has_uname);
    }
    if config.show & (1 << 6) != 0 {
        values[6].set(b"Unknown");
    }
    if config.show & (1 << 7) != 0 {
        values[7].set(b"Unknown");
    }
    if config.show & (1 << 8) != 0 {
        values[8].set(b"Unknown");
    }
    if config.show & (1 << 9) != 0 {
        hostname_value(&mut values[9]);
    }
    if config.show & (1 << 10) != 0 {
        values[10].set(b"Unknown");
    }
    if config.show & (1 << 11) != 0 {
        values[11].set(b"Unknown");
    }
    if config.show & (1 << 12) != 0 {
        values[12].set(b"Unknown");
    }
    if config.show & (1 << 13) != 0 {
        values[13].set(b"Unknown");
    }
    if config.show & (1 << 14) != 0 {
        values[14].set(b"Unknown");
    }
    if config.show & (1 << 15) != 0 {
        values[15].set(b"Unknown");
    }
    fill_unknown(&mut values);
    Info {
        values,
        cpu_usage: unknown(),
        gpu_usage: unknown(),
        elapsed_us: now().saturating_sub(start),
        rss_kb: 0,
    }
}

fn unknown() -> Field {
    let mut field = Field::new();
    field.set(b"N/A");
    field
}

fn copy_text(source: &[u8], field: &mut Field, available: bool) {
    field.clear();
    if !available {
        field.set(b"Unknown");
        return;
    }
    for &byte in source {
        if byte == 0 || field.len == field.data.len() {
            break;
        }
        field.push(byte);
    }
    field.trim();
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

fn env_text(name: &[u8], field: &mut Field) {
    field.clear();
    let pointer = unsafe { env_get(name.as_ptr()) };
    if pointer.is_null() {
        field.set(b"Unknown");
        return;
    }
    let mut index = 0;
    while index < field.data.len() - 1 {
        let byte = unsafe { *pointer.add(index) };
        if byte == 0 {
            break;
        }
        field.push(byte);
        index += 1;
    }
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

unsafe extern "C" {
    fn getenv(name: *const u8) -> *const u8;
}

unsafe fn env_get(name: *const u8) -> *const u8 {
    unsafe { getenv(name) }
}

fn hostname_value(field: &mut Field) {
    field.clear();
    unsafe { gethostname(field.data.as_mut_ptr(), field.data.len() - 1) };
    field.len = field.data.iter().position(|&byte| byte == 0).unwrap_or(0);
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

fn fill_unknown(values: &mut [Field; 16]) {
    for value in values {
        if value.len == 0 {
            value.set(b"Unknown");
        }
    }
}
