use crate::model::{
    Config, FIELD_BOARD, FIELD_CPU, FIELD_CPU_USAGE, FIELD_DE, FIELD_DISK, FIELD_DISTRO, FIELD_GPU,
    FIELD_HOST, FIELD_KERNEL, FIELD_LOAD, FIELD_MEMORY, FIELD_RESOLUTION, FIELD_SHELL, FIELD_TERM,
    FIELD_UPTIME, FIELD_WM, Field, Info, number,
};
use core::ffi::c_void;

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
    pub fn write(fd: i32, buffer: *const u8, length: usize) -> isize;
}

#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(path: *const u8, flags: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, name: *const u8) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> i32;
}

const RTLD_LAZY: i32 = 1;

#[repr(C)]
struct NvmlUtilization {
    gpu: u32,
    memory: u32,
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

pub fn collect(config: &Config, start: u64, gpu_usage: bool) -> Info {
    let mut values = [Field::new(); 16];
    let mut file = [0; 2048];
    let cpu_sample = if config.show & (FIELD_CPU | FIELD_CPU_USAGE) != 0 {
        read_cpu_ticks()
    } else {
        (0, 0)
    };
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
        wm_value(&mut desktop, &config.wm_override);
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
        resolution_value(&mut values[12]);
    }
    if config.show & FIELD_BOARD != 0 {
        let size = read_file(b"/sys/devices/virtual/dmi/id/board_name\0", &mut file);
        values[13].set(&file[..size]);
        values[13].trim();
        if values[13].len == 0 {
            values[13].set(b"Unknown");
        }
    }
    let rss_kb = if config.process_memory {
        let size = read_file(b"/proc/self/status\0", &mut file);
        let mut rss = Field::new();
        line_value(&file[..size], b"VmRSS:", &mut rss);
        rss.number()
    } else {
        0
    };
    let cpu_usage = if config.show & (FIELD_CPU | FIELD_CPU_USAGE) != 0 {
        cpu_usage_value(cpu_sample)
    } else {
        unknown()
    };
    if config.show & FIELD_CPU_USAGE != 0 {
        values[14] = cpu_usage;
    }
    let gpu_usage = if config.show & FIELD_GPU != 0 {
        gpu_usage_value(&values[6], gpu_usage)
    } else {
        unknown()
    };
    if config.show & FIELD_DE != 0 {
        de_value(&mut values[15]);
    }
    Info {
        values,
        cpu_usage,
        gpu_usage,
        elapsed_us: now().saturating_sub(start),
        rss_kb,
    }
}

fn unknown() -> Field {
    let mut field = Field::new();
    field.set(b"N/A");
    field
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

fn wm_value(field: &mut Field, override_value: &Field) {
    if override_value.len > 0 {
        field.set(&override_value.data[..override_value.len]);
        return;
    }
    field.set(b"Unknown");
    if env_copy(b"HYPRLAND_INSTANCE_SIGNATURE\0", &mut field.data) > 0 {
        field.set(b"Hyprland");
    } else if env_copy(b"SWAYSOCK\0", &mut field.data) > 0 {
        field.set(b"Sway");
    } else if env_copy(b"NIRI_SOCKET\0", &mut field.data) > 0 {
        field.set(b"niri");
    } else if env_copy(b"RIVER_SOCKET\0", &mut field.data) > 0 {
        field.set(b"river");
    } else if env_copy(b"I3SOCK\0", &mut field.data) > 0 {
        field.set(b"i3");
    } else if env_copy(b"BSPWM_SOCKET\0", &mut field.data) > 0 {
        field.set(b"bspwm");
    } else {
        for name in [
            b"XDG_CURRENT_DESKTOP\0".as_slice(),
            b"XDG_SESSION_DESKTOP\0".as_slice(),
            b"DESKTOP_SESSION\0".as_slice(),
        ] {
            let mut session = Field::new();
            env_value(name, &mut session);
            if session_wm(&session, field) {
                return;
            }
        }
        let mut desktop = Field::new();
        de_value(&mut desktop);
        if desktop.data[..desktop.len] == *b"GNOME" {
            field.set(b"Mutter");
        } else if desktop.data[..desktop.len] == *b"KDE Plasma"
            || desktop.data[..desktop.len] == *b"KDE"
        {
            field.set(b"KWin");
        } else if desktop.data[..desktop.len] == *b"XFCE" {
            field.set(b"Xfwm4");
        } else if desktop.data[..desktop.len] == *b"Cinnamon" {
            field.set(b"Muffin");
        } else if desktop.data[..desktop.len] == *b"MATE" {
            field.set(b"Marco");
        } else if desktop.data[..desktop.len] == *b"niri" {
            field.set(b"niri");
        } else if desktop.data[..desktop.len] == *b"river" {
            field.set(b"river");
        } else if desktop.data[..desktop.len] == *b"DWM" {
            field.set(b"dwm");
        } else if desktop.data[..desktop.len] == *b"dwl" {
            field.set(b"dwl");
        } else if desktop.data[..desktop.len] == *b"awesome" {
            field.set(b"awesome");
        } else if desktop.data[..desktop.len] == *b"openbox" {
            field.set(b"Openbox");
        } else if desktop.data[..desktop.len] == *b"xmonad" {
            field.set(b"XMonad");
        }
    }
}

fn session_wm(session: &Field, field: &mut Field) -> bool {
    let value = &session.data[..session.len];
    let name = if value == b"Hyprland" {
        Some(b"Hyprland".as_slice())
    } else if value == b"sway" || value == b"Sway" {
        Some(b"Sway".as_slice())
    } else if value == b"niri" {
        Some(b"niri".as_slice())
    } else if value == b"river" {
        Some(b"river".as_slice())
    } else if value == b"i3" {
        Some(b"i3".as_slice())
    } else if value == b"bspwm" {
        Some(b"bspwm".as_slice())
    } else if value == b"dwm" || value == b"DWM" {
        Some(b"dwm".as_slice())
    } else if value == b"dwl" {
        Some(b"dwl".as_slice())
    } else if value == b"awesome" {
        Some(b"awesome".as_slice())
    } else if value == b"openbox" || value == b"Openbox" {
        Some(b"Openbox".as_slice())
    } else if value == b"xmonad" || value == b"XMonad" {
        Some(b"XMonad".as_slice())
    } else {
        None
    };
    if let Some(name) = name {
        field.set(name);
        true
    } else {
        false
    }
}

fn de_value(field: &mut Field) {
    field.set(b"Unknown");
    for name in [
        b"XDG_CURRENT_DESKTOP\0".as_slice(),
        b"XDG_SESSION_DESKTOP\0".as_slice(),
        b"DESKTOP_SESSION\0".as_slice(),
    ] {
        env_value(name, field);
        if !field.is_unknown() && !is_compositor_name(field) {
            canonical_desktop(field);
            return;
        }
    }
    field.set(b"Unknown");
}

fn is_compositor_name(field: &Field) -> bool {
    field.data[..field.len] == *b"Hyprland"
        || field.data[..field.len] == *b"Sway"
        || field.data[..field.len] == *b"i3"
        || field.data[..field.len] == *b"bspwm"
        || field.data[..field.len] == *b"niri"
        || field.data[..field.len] == *b"river"
        || field.data[..field.len] == *b"DWM"
        || field.data[..field.len] == *b"dwl"
        || field.data[..field.len] == *b"awesome"
        || field.data[..field.len] == *b"openbox"
        || field.data[..field.len] == *b"xmonad"
}

fn canonical_desktop(field: &mut Field) {
    if field.contains(b"GNOME") {
        field.set(b"GNOME");
    } else if field.contains(b"KDE") || field.contains(b"Plasma") {
        field.set(b"KDE Plasma");
    } else if field.contains(b"XFCE") || field.contains(b"Xfce") {
        field.set(b"XFCE");
    } else if field.contains(b"Cinnamon") {
        field.set(b"Cinnamon");
    } else if field.contains(b"MATE") {
        field.set(b"MATE");
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
    field.clear();
    for key in [
        b"model name".as_slice(),
        b"Hardware".as_slice(),
        b"Model".as_slice(),
    ] {
        for line in data[..size].split(|byte| *byte == b'\n') {
            if line.starts_with(key) {
                if let Some(index) = line.iter().position(|byte| *byte == b':') {
                    let mut model = Field::new();
                    model.extend(&line[index + 1..]);
                    model.trim();
                    normalize_cpu(&mut model);
                    if model.len > 0 && !field.contains(&model.data[..model.len]) {
                        if field.len > 0 {
                            field.extend(b", ");
                        }
                        field.extend(&model.data[..model.len]);
                    }
                }
            }
        }
    }
    if field.len > 0 {
        return;
    }
    let mut fallback = [0; 128];
    let size = read_file(b"/sys/devices/virtual/dmi/id/product_name\0", &mut fallback);
    if size > 0 {
        field.set(&fallback[..size]);
        field.trim();
    }
    if field.len == 0 {
        field.set(b"Unknown");
    }
}

fn normalize_cpu(field: &mut Field) {
    for marker in [b"(R) ".as_slice(), b"(TM) ".as_slice()] {
        while let Some(index) = find_bytes(field, marker) {
            let end = index + marker.len();
            field.data.copy_within(end..field.len, index);
            field.len -= marker.len();
        }
    }
}

fn find_bytes(field: &Field, needle: &[u8]) -> Option<usize> {
    field.data[..field.len]
        .windows(needle.len())
        .position(|window| window == needle)
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

fn resolution_value(field: &mut Field) {
    field.set(b"Unknown");
    let connectors = [
        b"HDMI-A-1\0".as_slice(),
        b"HDMI-A-2\0".as_slice(),
        b"DP-1\0".as_slice(),
        b"DP-2\0".as_slice(),
        b"DP-3\0".as_slice(),
        b"DP-4\0".as_slice(),
        b"eDP-1\0".as_slice(),
        b"DSI-1\0".as_slice(),
        b"VGA-1\0".as_slice(),
        b"Virtual-1\0".as_slice(),
    ];
    for card in 0..8u8 {
        for connector in connectors {
            let mut path = [0; 112];
            let prefix = b"/sys/class/drm/card";
            let suffix = b"/modes\0";
            let mut len = 0;
            path[..prefix.len()].copy_from_slice(prefix);
            len += prefix.len();
            path[len] = b'0' + card;
            len += 1;
            path[len] = b'-';
            len += 1;
            let connector_len = connector.len() - 1;
            path[len..len + connector_len].copy_from_slice(&connector[..connector_len]);
            len += connector_len;
            path[len..len + suffix.len()].copy_from_slice(suffix);
            len += suffix.len();
            let mut status_path = [0; 112];
            let status_prefix = b"/sys/class/drm/card";
            let mut status_len = status_prefix.len();
            status_path[..status_len].copy_from_slice(status_prefix);
            status_path[status_len] = b'0' + card;
            status_len += 1;
            status_path[status_len] = b'-';
            status_len += 1;
            let connector_len = connector.len() - 1;
            status_path[status_len..status_len + connector_len]
                .copy_from_slice(&connector[..connector_len]);
            status_len += connector_len;
            let status_suffix = b"/status\0";
            status_path[status_len..status_len + status_suffix.len()]
                .copy_from_slice(status_suffix);
            let mut status = [0; 32];
            let status_size = read_file(
                &status_path[..status_len + status_suffix.len()],
                &mut status,
            );
            if status_size == 0 || !status[..status_size].starts_with(b"connected") {
                continue;
            }
            let mut data = [0; 128];
            let size = read_file(&path[..len], &mut data);
            if size == 0 {
                continue;
            }
            let mut end = data[..size]
                .iter()
                .position(|&byte| byte == b'\n')
                .unwrap_or(size);
            while end > 0 && data[end - 1].is_ascii_whitespace() {
                end -= 1;
            }
            if end > 0 {
                append_mode(field, &data[..end]);
            }
        }
    }
}

fn append_mode(field: &mut Field, mode: &[u8]) {
    let mut value = Field::new();
    value.set(mode);
    value.trim();
    if value.len == 0 || value.is_unknown() || field.contains(&value.data[..value.len]) {
        return;
    }
    if field.is_unknown() {
        field.clear();
    } else {
        field.extend(b", ");
    }
    field.extend(&value.data[..value.len]);
}

fn cpu_usage_value(sample: (u64, u64)) -> Field {
    let mut field = Field::new();
    if sample.0 == 0 {
        return unknown();
    }
    field.u64(sample.0.saturating_sub(sample.1) * 100 / sample.0);
    field.push(b'%');
    field
}

fn read_cpu_ticks() -> (u64, u64) {
    let mut data = [0; 512];
    let size = read_file(b"/proc/stat\0", &mut data);
    cpu_ticks(&data[..size])
}

fn gpu_usage_value(gpu: &Field, enabled: bool) -> Field {
    let mut output = Field::new();
    let mut found = false;
    for card in 0..8u8 {
        let mut path = [0; 96];
        let len = device_path(&mut path, card, b"/device/gpu_busy_percent\0");
        let mut data = [0; 32];
        let mut usage = Field::new();
        let size = read_file(&path[..len], &mut data);
        if size > 0 {
            usage = percentage_field(&data[..size]);
        }
        if usage.len == 0 {
            let len = device_path(&mut path, card, b"/gt_busy_percent\0");
            let size = read_file(&path[..len], &mut data);
            if size > 0 {
                usage = percentage_field(&data[..size]);
            }
        }
        if usage.len > 0 {
            usage.push(b'%');
            append_unique(&mut output, &usage, b", ");
            found = true;
        }
    }
    let nvidia = if enabled && gpu.contains(b"NVIDIA") {
        nvidia_usage_value()
    } else {
        Field::new()
    };
    if nvidia.len > 0 {
        append_unique(&mut output, &nvidia, b", ");
        found = true;
    }
    if found { output } else { unknown() }
}

fn nvidia_usage_value() -> Field {
    type Init = unsafe extern "C" fn() -> u32;
    type Shutdown = unsafe extern "C" fn() -> u32;
    type GetCount = unsafe extern "C" fn(*mut u32) -> u32;
    type GetHandle = unsafe extern "C" fn(u32, *mut *mut c_void) -> u32;
    type GetUtilization = unsafe extern "C" fn(*mut c_void, *mut NvmlUtilization) -> u32;

    let handle = unsafe { dlopen(b"libnvidia-ml.so.1\0".as_ptr(), RTLD_LAZY) };
    if handle.is_null() {
        return Field::new();
    }
    let init = unsafe { nvml_symbol::<Init>(handle, b"nvmlInit_v2\0".as_ptr()) };
    let shutdown = unsafe { nvml_symbol::<Shutdown>(handle, b"nvmlShutdown\0".as_ptr()) };
    let get_count = unsafe {
        nvml_symbol::<GetCount>(handle, b"nvmlDeviceGetCount_v2\0".as_ptr())
            .or_else(|| nvml_symbol::<GetCount>(handle, b"nvmlDeviceGetCount\0".as_ptr()))
    };
    let get_handle = unsafe {
        nvml_symbol::<GetHandle>(handle, b"nvmlDeviceGetHandleByIndex_v2\0".as_ptr())
            .or_else(|| nvml_symbol::<GetHandle>(handle, b"nvmlDeviceGetHandleByIndex\0".as_ptr()))
    };
    let get_utilization = unsafe {
        nvml_symbol::<GetUtilization>(handle, b"nvmlDeviceGetUtilizationRates\0".as_ptr())
    };
    let (Some(init), Some(shutdown), Some(get_count), Some(get_handle), Some(get_utilization)) =
        (init, shutdown, get_count, get_handle, get_utilization)
    else {
        unsafe { dlclose(handle) };
        return Field::new();
    };

    let mut count = 0;
    if unsafe { init() } != 0 || unsafe { get_count(&mut count) } != 0 {
        unsafe {
            shutdown();
            dlclose(handle);
        }
        return Field::new();
    }
    let mut output = Field::new();
    for index in 0..count {
        let mut device = core::ptr::null_mut();
        let mut utilization = NvmlUtilization { gpu: 0, memory: 0 };
        if unsafe { get_handle(index, &mut device) } == 0
            && unsafe { get_utilization(device, &mut utilization) } == 0
        {
            append_nvml_result(&mut output, 0, utilization.gpu);
        }
    }
    unsafe {
        shutdown();
        dlclose(handle);
    }
    output
}

fn append_nvml_result(output: &mut Field, status: u32, gpu: u32) {
    if status != 0 || gpu > 100 {
        return;
    }
    if output.len > 0 {
        output.extend(b", ");
    }
    output.u64(u64::from(gpu));
    output.push(b'%');
}

unsafe fn nvml_symbol<T>(handle: *mut c_void, name: *const u8) -> Option<T> {
    let pointer = unsafe { dlsym(handle, name) };
    if pointer.is_null() {
        None
    } else {
        Some(unsafe { core::mem::transmute_copy(&pointer) })
    }
}

fn is_percentage(field: &Field) -> bool {
    field.len > 0
        && field.data[..field.len]
            .iter()
            .all(|byte| byte.is_ascii_digit())
}

fn percentage_field(data: &[u8]) -> Field {
    let mut field = Field::new();
    field.set(data);
    field.trim();
    if !is_percentage(&field) {
        field.clear();
    }
    field
}

fn append_unique(output: &mut Field, value: &Field, separator: &[u8]) {
    if value.len == 0 || output.contains(&value.data[..value.len]) {
        return;
    }
    if output.len > 0 {
        output.extend(separator);
    }
    output.extend(&value.data[..value.len]);
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
    field.set(b"Unknown");
    let mut found = false;
    let mut uevent = [0; 512];
    let mut metadata = [0; 512];
    for card in 0..8u8 {
        let mut path = [0; 96];
        let len = device_path(&mut path, card, b"/device/uevent\0");
        let size = read_file(&path[..len], &mut uevent);
        if size == 0 {
            continue;
        }
        let mut driver = Field::new();
        line_value(&uevent[..size], b"DRIVER=", &mut driver);
        let mut model = Field::new();
        for suffix in [
            b"/device/product_name\0".as_slice(),
            b"/device/name\0".as_slice(),
            b"/device/label\0".as_slice(),
        ] {
            let model_len = device_path(&mut path, card, suffix);
            let model_size = read_file(&path[..model_len], &mut metadata);
            if model_size > 0 {
                model.set(&metadata[..model_size]);
                model.trim();
                if !model.is_unknown() {
                    break;
                }
            }
        }
        let mut detected = Field::new();
        if driver.data[..driver.len] == *b"nvidia" {
            let mut slot = Field::new();
            line_value(&uevent[..size], b"PCI_SLOT_NAME=", &mut slot);
            if !nvidia_model(&slot, &mut detected) {
                detected.set(b"NVIDIA");
            }
        } else if model.len > 0 && !model.is_unknown() {
            detected.set(&model.data[..model.len]);
        } else if driver.data[..driver.len] == *b"amdgpu" {
            detected.set(b"AMD Radeon");
        } else if driver.data[..driver.len] == *b"i915" || driver.data[..driver.len] == *b"xe" {
            detected.set(b"Intel Graphics");
        }
        if detected.len > 0 && !detected.is_unknown() {
            normalize_gpu(&mut detected);
            if !found {
                field.clear();
                found = true;
            } else {
                field.extend(b", ");
            }
            field.extend(&detected.data[..detected.len]);
        }
    }
    if !found {
        field.set(b"Unknown");
    }
}

fn normalize_gpu(field: &mut Field) {
    for marker in [b"(R) ".as_slice(), b"(TM) ".as_slice()] {
        while let Some(index) = find_bytes(field, marker) {
            let end = index + marker.len();
            field.data.copy_within(end..field.len, index);
            field.len -= marker.len();
        }
    }
}

fn device_path(path: &mut [u8], card: u8, suffix: &[u8]) -> usize {
    let prefix = b"/sys/class/drm/card";
    let mut len = prefix.len();
    path[..len].copy_from_slice(prefix);
    path[len] = b'0' + card;
    len += 1;
    path[len..len + suffix.len()].copy_from_slice(suffix);
    len + suffix.len()
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

#[cfg(test)]
mod tests {
    use super::{
        append_mode, append_nvml_result, append_unique, cpu_ticks, is_percentage, normalize_cpu,
        percentage_field,
    };
    use crate::model::Field;

    #[test]
    fn parses_cpu_ticks_from_proc_stat() {
        assert_eq!(cpu_ticks(b"cpu  10 20 30 40 5 6\ncpu0 1 2"), (111, 45));
    }

    #[test]
    fn accepts_only_numeric_gpu_usage() {
        let mut field = Field::new();
        field.set(b"42");
        assert!(is_percentage(&field));
        field.set(b"N/A");
        assert!(!is_percentage(&field));
    }

    #[test]
    fn parses_gpu_sysfs_fixture_values() {
        let busy = percentage_field(b" 37\n");
        assert_eq!(&busy.data[..busy.len], b"37");
        assert!(percentage_field(b"not available").len == 0);
    }

    #[test]
    fn combines_multiple_gpu_fixture_values_without_duplicates() {
        let mut output = Field::new();
        let first = percentage_field(b"12");
        let second = percentage_field(b"88");
        append_unique(&mut output, &first, b", ");
        append_unique(&mut output, &second, b", ");
        append_unique(&mut output, &first, b", ");
        assert_eq!(&output.data[..output.len], b"12, 88");
    }

    #[test]
    fn combines_connected_resolution_fixture_modes() {
        let mut output = Field::new();
        output.set(b"Unknown");
        append_mode(&mut output, b"1920x1080\n");
        append_mode(&mut output, b"2560x1440\n");
        append_mode(&mut output, b"1920x1080\n");
        assert_eq!(&output.data[..output.len], b"1920x1080, 2560x1440");
    }

    #[test]
    fn accepts_valid_nvml_fixture_result() {
        let mut output = Field::new();
        append_nvml_result(&mut output, 0, 73);
        assert_eq!(&output.data[..output.len], b"73%");
    }

    #[test]
    fn rejects_failed_or_invalid_nvml_fixture_result() {
        let mut output = Field::new();
        append_nvml_result(&mut output, 999, 73);
        append_nvml_result(&mut output, 0, 101);
        assert_eq!(output.len, 0);
    }

    #[test]
    fn normalizes_cpu_vendor_markers() {
        let mut field = Field::new();
        field.set(b"AMD (R) Ryzen (TM) 7");
        normalize_cpu(&mut field);
        assert_eq!(&field.data[..field.len], b"AMD Ryzen 7");
    }
}
