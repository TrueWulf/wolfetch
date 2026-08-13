use crate::model::{Config, Field, Info};

pub fn now() -> u64 {
    0
}
pub fn read_file(_: &[u8], _: &mut [u8]) -> usize {
    0
}
pub fn env_copy(_: &[u8], _: &mut [u8]) -> usize {
    0
}
pub fn copy_cstr(_: *const u8, _: &mut [u8]) -> usize {
    0
}
pub unsafe fn isatty_stdout() -> bool {
    false
}
pub unsafe fn write(_: i32, _: *const u8, _: usize) -> isize {
    -1
}
pub fn os_label() -> &'static [u8] {
    b"OS"
}

pub fn collect(_: &Config, _: u64) -> Info {
    let mut values = [Field::new(); 16];
    for value in &mut values {
        value.set(b"Unknown");
    }
    Info {
        values,
        cpu_usage: unknown(),
        gpu_usage: unknown(),
        elapsed_us: 0,
        rss_kb: 0,
    }
}

fn unknown() -> Field {
    let mut field = Field::new();
    field.set(b"N/A");
    field
}
