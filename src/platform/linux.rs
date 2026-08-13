use crate::model::{
    Config, FIELD_BOARD, FIELD_CPU, FIELD_CPU_USAGE, FIELD_DISK, FIELD_DISTRO, FIELD_GPU,
    FIELD_HOST, FIELD_KERNEL, FIELD_LOAD, FIELD_MEMORY, FIELD_RESOLUTION, FIELD_SHELL, FIELD_TERM,
    FIELD_UPTIME, FIELD_WM, Field, Info, number,
};

#[link(name = "c")]
unsafe extern "C" {
    fn clock_gettime(clock: i32, time: *mut Timespec) -> i32;
    fn close(fd: i32) -> i32;
    fn getenv(name: *const u8) -> *const u8;
    fn isatty(fd: i32) -> i32;
    fn open(path: *const u8, flags: i32) -> i32;
    fn read(fd: i32, buffer: *mut u8, length: usize) -> isize;
    fn gethostname(name: *mut u8, length: usize) -> i32;
    fn statvfs(path: *const u8, info: *mut Statvfs) -> i32;
    fn nanosleep(request: *const Timespec, remain: *mut Timespec) -> i32;
    pub fn write(fd: i32, buffer: *const u8, length: usize) -> isize;
}

#[repr(C)]
struct Timespec {
    seconds: i64,
    nanos: i64,
}

#[repr(C)]
struct Statvfs {
    block_size: u64,
    fragment_size: u64,
    blocks: u64,
    free_blocks: u64,
    available_blocks: u64,
    files: u64,
    free_files: u64,
    available_files: u64,
    filesystem_id: u64,
    flags: u64,
    spare: [u64; 4],
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

pub fn os_label() -> &'static [u8] {
    b"Distro"
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
    let mut length = 0;
    while length + 1 < output.len() {
        let byte = unsafe { *pointer.add(length) };
        output[length] = byte;
        if byte == 0 {
            return length + 1;
        }
        length += 1;
    }
    0
}

pub fn collect(config: &Config, start: u64) -> Info {
    let mut values = [Field::new(); 15];
    let mut file = [0; 2048];
    if config.show & FIELD_DISTRO != 0 {
        let size = read_file(b"/etc/os-release\0", &mut file);
        line_value(&file[..size], b"NAME=", &mut values[0]);
    }
    if config.show & FIELD_KERNEL != 0 {
        let size = read_file(b"/proc/sys/kernel/osrelease\0", &mut file);
        values[1].set(&file[..size]);
        values[1].trim();
    }
    let mut desktop = Field::new();
    if config.show & FIELD_WM != 0 {
        env_value(b"XDG_CURRENT_DESKTOP\0", &mut desktop);
        if desktop.len == 0 || desktop.data[..desktop.len] == *b"Unknown" {
            env_value(b"DESKTOP_SESSION\0", &mut desktop);
        }
        values[2].set(&desktop.data[..desktop.len]);
    }
    if config.show & FIELD_TERM != 0 {
        env_value(b"TERM\0", &mut values[3]);
    }
    if config.show & FIELD_SHELL != 0 {
        env_value(b"SHELL\0", &mut values[4]);
        values[4].slash_tail();
    }
    if config.show & FIELD_CPU != 0 {
        read_cpu(&mut values[5]);
    }
    if config.show & FIELD_GPU != 0 {
        gpu_value(&mut values[6]);
    }
    if config.show & FIELD_MEMORY != 0 {
        let size = read_file(b"/proc/meminfo\0", &mut file);
        memory_value(&file[..size], &mut values[7]);
    }
    if config.show & FIELD_UPTIME != 0 {
        let size = read_file(b"/proc/uptime\0", &mut file);
        uptime_value(&file[..size], &mut values[8]);
    }
    if config.show & FIELD_HOST != 0 {
        hostname_value(&mut values[9]);
    }
    if config.show & FIELD_LOAD != 0 {
        let size = read_file(b"/proc/loadavg\0", &mut file);
        load_value(&file[..size], &mut values[10]);
    }
    if config.show & FIELD_DISK != 0 {
        disk_value(&mut values[11]);
    }
    if config.show & FIELD_RESOLUTION != 0 {
        let size = read_file(b"/sys/class/graphics/fb0/virtual_size\0", &mut file);
        resolution_value(&file[..size], &mut values[12]);
    }
    if config.show & FIELD_BOARD != 0 {
        let size = read_file(b"/sys/devices/virtual/dmi/id/board_name\0", &mut file);
        values[13].set(&file[..size]);
        values[13].trim();
        if values[13].len == 0 {
            values[13].set(b"Unknown");
        }
    }
    if config.show & FIELD_CPU_USAGE != 0 {
        cpu_usage_value(&mut values[14]);
    }
    let rss_kb = if config.process_memory {
        let size = read_file(b"/proc/self/status\0", &mut file);
        let mut rss = Field::new();
        line_value(&file[..size], b"VmRSS:", &mut rss);
        rss.number()
    } else {
        0
    };
    let wm = if desktop.contains(b"Hypr") || desktop.contains(b"i3") {
        b"WM"
    } else {
        b"DE"
    };
    Info {
        values,
        wm,
        elapsed_us: now().saturating_sub(start),
        rss_kb,
    }
}

fn env_value(name: &[u8], field: &mut Field) {
    field.clear();
    let pointer = unsafe { getenv(name.as_ptr()) };
    let length = copy_cstr(pointer, &mut field.data);
    field.len = length.saturating_sub(1);
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

fn line_value(data: &[u8], key: &[u8], field: &mut Field) {
    field.clear();
    for line in data.split(|byte| *byte == b'\n') {
        if let Some(value) = line.strip_prefix(key) {
            field.extend(value);
            field.trim();
            return;
        }
    }
    field.set(b"Unknown");
}

fn read_cpu(field: &mut Field) {
    let mut data = [0; 512];
    let size = read_file(b"/proc/cpuinfo\0", &mut data);
    for line in data[..size].split(|byte| *byte == b'\n') {
        if line.starts_with(b"model name") {
            if let Some(index) = line.iter().position(|byte| *byte == b':') {
                field.extend(&line[index + 1..]);
                field.trim();
                return;
            }
        }
    }
    field.set(b"Unknown");
}

fn memory_value(data: &[u8], field: &mut Field) {
    let mut total = 0;
    let mut available = 0;
    for line in data.split(|byte| *byte == b'\n') {
        if line.starts_with(b"MemTotal:") {
            total = number(line);
        }
        if line.starts_with(b"MemAvailable:") {
            available = number(line);
        }
    }
    let used = total.saturating_sub(available);
    field.clear();
    field.decimal(used * 100 / (1024 * 1024));
    field.extend(b" / ");
    field.append_decimal(total * 100 / (1024 * 1024));
    field.extend(b" GiB (");
    field.u64(if total == 0 { 0 } else { used * 100 / total });
    field.extend(b"%)");
}

fn uptime_value(data: &[u8], field: &mut Field) {
    let seconds = number(data);
    let days = seconds / 86400;
    let hours = seconds / 3600 % 24;
    let minutes = seconds / 60 % 60;
    field.clear();
    if days > 0 {
        field.u64(days);
        field.extend(b"d ");
    }
    if hours > 0 || days > 0 {
        field.u64(hours);
        field.extend(b"h ");
    }
    field.u64(minutes);
    field.push(b'm');
}

fn hostname_value(field: &mut Field) {
    field.clear();
    unsafe { gethostname(field.data.as_mut_ptr(), field.data.len() - 1) };
    field.len = field
        .data
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(field.data.len() - 1);
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

fn load_value(data: &[u8], field: &mut Field) {
    field.clear();
    if let Some(end) = data.iter().position(|&byte| byte == b' ') {
        field.extend(&data[..end]);
    } else {
        field.extend(data);
    }
    field.trim();
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

fn disk_value(field: &mut Field) {
    let mut info = Statvfs {
        block_size: 0,
        fragment_size: 0,
        blocks: 0,
        free_blocks: 0,
        available_blocks: 0,
        files: 0,
        free_files: 0,
        available_files: 0,
        filesystem_id: 0,
        flags: 0,
        spare: [0; 4],
    };
    if unsafe { statvfs(b"/\0".as_ptr(), &mut info) } != 0 || info.blocks == 0 {
        field.set(b"Unknown");
        return;
    }
    let used = info.blocks.saturating_sub(info.available_blocks);
    field.clear();
    field.u64(used * 100 / info.blocks);
    field.extend(b"% used");
}

fn resolution_value(data: &[u8], field: &mut Field) {
    let Some(comma) = data.iter().position(|&byte| byte == b',') else {
        field.set(b"Unknown");
        return;
    };
    field.clear();
    field.extend(&data[..comma]);
    field.push(b'x');
    field.extend(&data[comma + 1..]);
    field.trim();
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

fn cpu_usage_value(field: &mut Field) {
    let mut before = [0; 512];
    let first_size = read_file(b"/proc/stat\0", &mut before);
    let sleep = Timespec {
        seconds: 0,
        nanos: 20_000_000,
    };
    unsafe { nanosleep(&sleep, core::ptr::null_mut()) };
    let mut after = [0; 512];
    let second_size = read_file(b"/proc/stat\0", &mut after);
    let (before_total, before_idle) = cpu_ticks(&before[..first_size]);
    let (after_total, after_idle) = cpu_ticks(&after[..second_size]);
    let total = after_total.saturating_sub(before_total);
    let idle = after_idle.saturating_sub(before_idle);
    if total == 0 {
        field.set(b"Unknown");
        return;
    }
    field.clear();
    field.u64(total.saturating_sub(idle) * 100 / total);
    field.push(b'%');
}

fn cpu_ticks(data: &[u8]) -> (u64, u64) {
    let Some(line) = data.split(|&byte| byte == b'\n').next() else {
        return (0, 0);
    };
    let mut values = [0; 8];
    let mut count = 0;
    for part in line.split(|&byte| byte == b' ') {
        if count == values.len() {
            break;
        }
        if part.is_empty() || part == b"cpu" {
            continue;
        }
        values[count] = number(part);
        count += 1;
    }
    if count < 5 {
        return (0, 0);
    }
    (values[..count].iter().sum(), values[3] + values[4])
}

fn gpu_value(field: &mut Field) {
    let mut data = [0; 256];
    let mut size = read_file(b"/sys/class/drm/card0/device/uevent\0", &mut data);
    if size == 0 {
        size = read_file(b"/sys/class/drm/card1/device/uevent\0", &mut data);
    }
    let mut driver = Field::new();
    line_value(&data[..size], b"DRIVER=", &mut driver);
    if driver.data[..driver.len] == *b"nvidia" {
        let mut slot = Field::new();
        line_value(&data[..size], b"PCI_SLOT_NAME=", &mut slot);
        if !nvidia_model(&slot, field) {
            field.set(b"NVIDIA");
        }
    } else if driver.data[..driver.len] == *b"amdgpu" {
        field.set(b"AMD Radeon");
    } else if driver.data[..driver.len] == *b"i915" || driver.data[..driver.len] == *b"xe" {
        field.set(b"Intel Graphics");
    } else {
        field.set(b"Unknown");
    }
}

fn nvidia_model(slot: &Field, field: &mut Field) -> bool {
    if slot.len == 0 {
        return false;
    }
    let mut path = [0; 128];
    let prefix = b"/proc/driver/nvidia/gpus/";
    let suffix = b"/information\0";
    let mut length = 0;
    for &byte in prefix {
        path[length] = byte;
        length += 1;
    }
    for &byte in &slot.data[..slot.len] {
        if length + 1 >= path.len() {
            return false;
        }
        path[length] = byte;
        length += 1;
    }
    for &byte in suffix {
        if length >= path.len() {
            return false;
        }
        path[length] = byte;
        length += 1;
    }
    let mut data = [0; 512];
    let size = read_file(&path[..length], &mut data);
    if size == 0 {
        return false;
    }
    line_value(&data[..size], b"Model:", field);
    field.len > 0 && field.data[..field.len] != *b"Unknown"
}
