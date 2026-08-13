use crate::art;
use crate::model::{
    Config, FIELD_BOARD, FIELD_CPU, FIELD_CPU_USAGE, FIELD_DE, FIELD_DISK, FIELD_DISTRO, FIELD_GPU,
    FIELD_HOST, FIELD_KERNEL, FIELD_LOAD, FIELD_MEMORY, FIELD_RESOLUTION, FIELD_SHELL, FIELD_TERM,
    FIELD_UPTIME, FIELD_WM, Field, Info, VERSION,
};
use crate::platform;

pub fn help() {
    let mut buffer = Buffer::new();
    buffer.extend(b"wolfetch ");
    buffer.extend(VERSION);
    buffer.extend(b"\n\nUsage: wolfetch [OPTIONS]\n\nOptions:\n  -p, --plain      Disable colors\n  -j, --json       Print JSON\n  -n, --no-logo    Disable ASCII art\n  -f, --fast       Skip CPU, GPU and process memory\n  -m, --minimal    Compact output without logo or heavy fields\n      --theme NAME Use a built-in theme\n      --config PATH Use a config file\n  -V, --version    Print version\n  -h, --help       Print help\n");
    buffer.write();
}

pub fn version() {
    let mut buffer = Buffer::new();
    buffer.extend(b"wolfetch ");
    buffer.extend(VERSION);
    buffer.push(b'\n');
    buffer.write();
}

pub fn error() {
    let mut buffer = Buffer::new();
    buffer.extend(b"wolfetch: unknown or incomplete option\n");
    buffer.write_fd(2);
}

pub fn render(info: &Info, config: &Config, plain: bool, no_logo: bool, json: bool) {
    if json {
        render_json(info, config);
        return;
    }
    let mut buffer = Buffer::new();
    let colors = !plain && unsafe { platform::isatty_stdout() };
    let logo = config.logo && !no_logo;
    let labels: [&[u8]; 16] = [
        platform::os_label(),
        b"Kernel",
        b"WM",
        b"Term",
        b"Shell",
        b"CPU",
        b"GPU",
        b"Memory",
        b"Uptime",
        b"Host",
        b"Load",
        b"Disk",
        b"Resolution",
        b"Board",
        b"CPU usage",
        b"DE",
    ];
    let masks = [
        FIELD_DISTRO,
        FIELD_KERNEL,
        FIELD_WM,
        FIELD_TERM,
        FIELD_SHELL,
        FIELD_CPU,
        FIELD_GPU,
        FIELD_MEMORY,
        FIELD_UPTIME,
        FIELD_HOST,
        FIELD_LOAD,
        FIELD_DISK,
        FIELD_RESOLUTION,
        FIELD_BOARD,
        FIELD_CPU_USAGE,
        FIELD_DE,
    ];
    let mut label_width = 7;
    for position in 0..config.order_len {
        let width = labels[config.order[position] as usize].len() + 1;
        if width > label_width {
            label_width = width;
        }
    }
    let mut service = Field::new();
    if config.runtime {
        service.append_ms(info.elapsed_us);
    }
    if config.runtime && config.process_memory {
        service.extend(b" | ");
    }
    if config.process_memory {
        if info.rss_kb == 0 {
            service.extend(b"n/a");
        } else {
            service.append_decimal(info.rss_kb * 100 / 1024);
            service.extend(b" MiB");
        }
    }
    let mut service_rendered = false;
    for position in 0..config.order_len {
        let index = config.order[position] as usize;
        if logo && index < 9 {
            let art_bytes = art::ART[index];
            write_art_line(&mut buffer, art_bytes, config, index, colors);
        }
        if config.show & masks[index] != 0 {
            if logo && index < 9 {
                buffer.extend(b"  ");
            }
            label(
                &mut buffer,
                labels[index],
                label_width,
                config.theme.label,
                colors,
            );
            value(&mut buffer, &info.values[index], config.theme.value, colors);
            buffer.push(b'\n');
            if index == 8 && service.len > 0 {
                write_service(&mut buffer, &service, config, logo, colors);
                service_rendered = true;
            }
        } else if logo && index < 9 {
            buffer.push(b'\n');
        }
    }
    if service.len > 0 && !service_rendered {
        write_service(&mut buffer, &service, config, logo, colors);
    }
    if logo {
        write_art_line(&mut buffer, art::ART[11], config, 11, colors);
        buffer.push(b'\n');
    }
    buffer.write();
}

fn write_service(buffer: &mut Buffer, service: &Field, config: &Config, logo: bool, colors: bool) {
    if logo {
        write_art_line(buffer, art::ART[10], config, 10, colors);
        buffer.extend(b" ");
    }
    stats(buffer, service, config.theme.stats, colors);
    buffer.push(b'\n');
}

fn write_art_line(
    buffer: &mut Buffer,
    art_bytes: &[u8],
    config: &Config,
    index: usize,
    colors: bool,
) {
    if colors {
        buffer.color(art_color(config, index));
    }
    buffer.extend(art_bytes);
    buffer.spaces(22usize.saturating_sub(art::width(art_bytes)));
    if colors {
        buffer.reset();
    }
}

fn art_color(config: &Config, index: usize) -> u8 {
    let _ = index;
    config.theme.art_light
}

fn label(buffer: &mut Buffer, text: &[u8], width: usize, color: u8, colors: bool) {
    if colors {
        buffer.color(color);
    }
    buffer.extend(text);
    buffer.spaces(width.saturating_sub(text.len()));
    buffer.extend(b": ");
    if colors {
        buffer.reset();
    }
}

fn value(buffer: &mut Buffer, field: &Field, color: u8, colors: bool) {
    if colors {
        buffer.color(color);
    }
    buffer.extend(&field.data[..field.len]);
    if field.truncated {
        buffer.extend(b"...");
    }
    if colors {
        buffer.reset();
    }
}

fn stats(buffer: &mut Buffer, field: &Field, color: u8, colors: bool) {
    if colors {
        buffer.color(color);
    }
    buffer.extend(&field.data[..field.len]);
    if field.truncated {
        buffer.extend(b"...");
    }
    if colors {
        buffer.reset();
    }
}

fn render_json(info: &Info, config: &Config) {
    let mut buffer = Buffer::new();
    buffer.push(b'{');
    let labels: [&[u8]; 16] = [
        platform::os_label(),
        b"Kernel",
        b"WM",
        b"Term",
        b"Shell",
        b"CPU",
        b"GPU",
        b"Memory",
        b"Uptime",
        b"Host",
        b"Load",
        b"Disk",
        b"Resolution",
        b"Board",
        b"CPU usage",
        b"DE",
    ];
    let masks = [
        FIELD_DISTRO,
        FIELD_KERNEL,
        FIELD_WM,
        FIELD_TERM,
        FIELD_SHELL,
        FIELD_CPU,
        FIELD_GPU,
        FIELD_MEMORY,
        FIELD_UPTIME,
        FIELD_HOST,
        FIELD_LOAD,
        FIELD_DISK,
        FIELD_RESOLUTION,
        FIELD_BOARD,
        FIELD_CPU_USAGE,
        FIELD_DE,
    ];
    let mut first = true;
    for position in 0..config.order_len {
        let index = config.order[position] as usize;
        if config.show & masks[index] == 0 {
            continue;
        }
        if !first {
            buffer.push(b',');
        }
        first = false;
        buffer.push(b'"');
        json_bytes(&mut buffer, labels[index]);
        buffer.extend(b"\":\"");
        json_bytes(
            &mut buffer,
            &info.values[index].data[..info.values[index].len],
        );
        if info.values[index].truncated {
            buffer.extend(b"...");
        }
        buffer.push(b'"');
    }
    if config.runtime {
        if !first {
            buffer.push(b',');
        }
        buffer.extend(b"\"startup_ms\":");
        let mut field = Field::new();
        field.append_decimal(info.elapsed_us / 10);
        buffer.extend(&field.data[..field.len]);
    }
    if config.process_memory {
        buffer.extend(b",\"process_memory_mib\":");
        if info.rss_kb == 0 {
            buffer.extend(b"null");
        } else {
            let mut field = Field::new();
            field.decimal(info.rss_kb * 100 / 1024);
            buffer.extend(&field.data[..field.len]);
        }
    }
    buffer.extend(b"}\n");
    buffer.write();
}

fn json_bytes(buffer: &mut Buffer, bytes: &[u8]) {
    for &byte in bytes {
        match byte {
            b'"' => buffer.extend(b"\\\""),
            b'\\' => buffer.extend(b"\\\\"),
            b'\n' => buffer.extend(b"\\n"),
            b'\r' => buffer.extend(b"\\r"),
            b'\t' => buffer.extend(b"\\t"),
            0x20..=0x7e => buffer.push(byte),
            _ => buffer.push(b'?'),
        }
    }
}

struct Buffer {
    data: [u8; 4096],
    len: usize,
    truncated: bool,
}

impl Buffer {
    const fn new() -> Self {
        Self {
            data: [0; 4096],
            len: 0,
            truncated: false,
        }
    }
    fn push(&mut self, byte: u8) {
        if self.len < self.data.len() {
            self.data[self.len] = byte;
            self.len += 1;
        } else {
            self.truncated = true;
        }
    }
    fn extend(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.push(byte);
        }
    }
    fn spaces(&mut self, count: usize) {
        for _ in 0..count {
            self.push(b' ');
        }
    }
    fn color(&mut self, code: u8) {
        self.extend(b"\x1b[38;5;");
        self.number(code as u64);
        self.push(b'm');
    }
    fn reset(&mut self) {
        self.extend(b"\x1b[0m");
    }
    fn number(&mut self, mut value: u64) {
        let mut digits = [0; 3];
        let mut length = 0;
        if value == 0 {
            self.push(b'0');
            return;
        }
        while value > 0 {
            digits[length] = b'0' + (value % 10) as u8;
            value /= 10;
            length += 1;
        }
        while length > 0 {
            length -= 1;
            self.push(digits[length]);
        }
    }
    fn write(&self) {
        self.write_fd(1);
    }
    fn write_fd(&self, fd: i32) {
        let mut offset = 0;
        while offset < self.len {
            let size =
                unsafe { platform::write(fd, self.data[offset..].as_ptr(), self.len - offset) };
            if size <= 0 {
                return;
            }
            offset += size as usize;
        }
        if self.truncated {
            unsafe {
                platform::write(2, b"wolfetch: output truncated\n".as_ptr(), 27);
            }
        }
    }
}
