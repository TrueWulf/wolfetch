pub const VERSION: &[u8] = b"0.5.0";
pub const FIELD_DISTRO: u16 = 1 << 0;
pub const FIELD_KERNEL: u16 = 1 << 1;
pub const FIELD_WM: u16 = 1 << 2;
pub const FIELD_TERM: u16 = 1 << 3;
pub const FIELD_SHELL: u16 = 1 << 4;
pub const FIELD_CPU: u16 = 1 << 5;
pub const FIELD_GPU: u16 = 1 << 6;
pub const FIELD_MEMORY: u16 = 1 << 7;
pub const FIELD_UPTIME: u16 = 1 << 8;
pub const FIELD_HOST: u16 = 1 << 9;
pub const FIELD_LOAD: u16 = 1 << 10;
pub const FIELD_DISK: u16 = 1 << 11;
pub const FIELD_RESOLUTION: u16 = 1 << 12;
pub const FIELD_BOARD: u16 = 1 << 13;
pub const FIELD_CPU_USAGE: u16 = 1 << 14;
pub const FIELD_DEFAULT: u16 = FIELD_DISTRO
    | FIELD_KERNEL
    | FIELD_WM
    | FIELD_TERM
    | FIELD_SHELL
    | FIELD_CPU
    | FIELD_GPU
    | FIELD_MEMORY
    | FIELD_UPTIME;
pub const FIELD_ALL: u16 = FIELD_DEFAULT
    | FIELD_HOST
    | FIELD_LOAD
    | FIELD_DISK
    | FIELD_RESOLUTION
    | FIELD_BOARD
    | FIELD_CPU_USAGE;

#[derive(Clone, Copy)]
pub struct Field {
    pub data: [u8; 128],
    pub len: usize,
}

impl Field {
    pub const fn new() -> Self {
        Self {
            data: [0; 128],
            len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn push(&mut self, byte: u8) {
        if self.len < self.data.len() {
            self.data[self.len] = byte;
            self.len += 1;
        }
    }

    pub fn extend(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.push(byte);
        }
    }

    pub fn set(&mut self, bytes: &[u8]) {
        self.clear();
        self.extend(bytes);
    }

    pub fn trim(&mut self) {
        let mut start = 0;
        let mut end = self.len;
        while start < end && self.data[start].is_ascii_whitespace() {
            start += 1;
        }
        while end > start && self.data[end - 1].is_ascii_whitespace() {
            end -= 1;
        }
        if end > start + 1 && self.data[start] == b'"' && self.data[end - 1] == b'"' {
            start += 1;
            end -= 1;
        }
        self.data.copy_within(start..end, 0);
        self.len = end - start;
    }

    pub fn contains(&self, needle: &[u8]) -> bool {
        self.data[..self.len]
            .windows(needle.len())
            .any(|window| window == needle)
    }

    pub fn slash_tail(&mut self) {
        if let Some(index) = self.data[..self.len].iter().rposition(|&byte| byte == b'/') {
            self.data.copy_within(index + 1..self.len, 0);
            self.len -= index + 1;
        }
    }

    pub fn number(&self) -> u64 {
        number(&self.data[..self.len])
    }

    pub fn decimal(&mut self, hundredths: u64) {
        self.clear();
        self.append_decimal(hundredths);
    }

    pub fn append_decimal(&mut self, hundredths: u64) {
        self.u64(hundredths / 100);
        self.push(b'.');
        self.push(b'0' + (hundredths / 10 % 10) as u8);
        self.push(b'0' + (hundredths % 10) as u8);
    }

    pub fn append_ms(&mut self, micros: u64) {
        self.u64(micros / 1000);
        self.push(b'.');
        self.push(b'0' + (micros / 100 % 10) as u8);
        self.push(b'0' + (micros / 10 % 10) as u8);
        self.extend(b" ms");
    }

    pub fn u64(&mut self, mut number: u64) {
        let mut digits = [0; 20];
        let mut len = 0;
        if number == 0 {
            self.push(b'0');
            return;
        }
        while number > 0 {
            digits[len] = b'0' + (number % 10) as u8;
            number /= 10;
            len += 1;
        }
        while len > 0 {
            len -= 1;
            self.push(digits[len]);
        }
    }
}

pub fn number(bytes: &[u8]) -> u64 {
    let mut value = 0;
    for &byte in bytes {
        if byte.is_ascii_digit() {
            value = value * 10 + u64::from(byte - b'0');
        } else if value > 0 {
            break;
        }
    }
    value
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub art: u8,
    pub art_light: u8,
    pub shadow: u8,
    pub deep_shadow: u8,
    pub label: u8,
    pub value: u8,
    pub stats: u8,
}

impl Palette {
    pub const fn royal() -> Self {
        Self {
            art: 27,
            art_light: 75,
            shadow: 243,
            deep_shadow: 238,
            label: 75,
            value: 255,
            stats: 243,
        }
    }

    pub const fn mono() -> Self {
        Self {
            art: 255,
            art_light: 255,
            shadow: 243,
            deep_shadow: 238,
            label: 255,
            value: 255,
            stats: 243,
        }
    }

    pub const fn ocean() -> Self {
        Self {
            art: 31,
            art_light: 81,
            shadow: 243,
            deep_shadow: 238,
            label: 81,
            value: 255,
            stats: 243,
        }
    }

    pub const fn gray() -> Self {
        Self {
            art: 244,
            art_light: 250,
            shadow: 245,
            deep_shadow: 238,
            label: 250,
            value: 255,
            stats: 245,
        }
    }
}

pub struct Config {
    pub theme: Palette,
    pub show: u16,
    pub logo: bool,
    pub runtime: bool,
    pub process_memory: bool,
    pub order: [u8; 15],
    pub order_len: usize,
}

impl Config {
    pub const fn new() -> Self {
        Self {
            theme: Palette::royal(),
            show: FIELD_DEFAULT,
            logo: true,
            runtime: true,
            process_memory: true,
            order: [0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0],
            order_len: 9,
        }
    }
}

pub struct Info {
    pub values: [Field; 15],
    pub wm: &'static [u8],
    pub elapsed_us: u64,
    pub rss_kb: u64,
}

pub struct Args {
    pub plain: bool,
    pub json: bool,
    pub no_logo: bool,
    pub fast: bool,
    pub theme: *const u8,
    pub config: *const u8,
    pub help: bool,
    pub version: bool,
    pub error: bool,
}
