use crate::model::{
    Config, FIELD_ALL, FIELD_BOARD, FIELD_CPU, FIELD_CPU_USAGE, FIELD_DE, FIELD_DISK, FIELD_DISTRO,
    FIELD_GPU, FIELD_HOST, FIELD_KERNEL, FIELD_LOAD, FIELD_MEMORY, FIELD_RESOLUTION, FIELD_SHELL,
    FIELD_TERM, FIELD_UPTIME, FIELD_WM, Palette,
};
use crate::platform;

pub fn load(config: &mut Config, explicit: *const u8) {
    let mut path = [0; 256];
    let length = if !explicit.is_null() {
        platform::copy_cstr(explicit, &mut path)
    } else {
        default_path(&mut path)
    };
    if length == 0 {
        return;
    }
    let mut data = [0; 2048];
    let size = platform::read_file(&path[..length], &mut data);
    for line in data[..size].split(|byte| *byte == b'\n') {
        apply_line(config, line);
    }
}

pub fn theme_arg(config: &mut Config, pointer: *const u8) {
    if pointer.is_null() {
        return;
    }
    let mut value = [0; 32];
    let length = platform::copy_cstr(pointer, &mut value);
    set_theme(config, &value[..length]);
}

fn default_path(path: &mut [u8]) -> usize {
    let mut base = [0; 192];
    let mut length = platform::env_copy(b"XDG_CONFIG_HOME\0", &mut base);
    if length == 0 {
        length = platform::env_copy(b"HOME\0", &mut base);
        let suffix = b"/.config/wolfetch/config\0";
        if length == 0 || length + suffix.len() > path.len() {
            return 0;
        }
        length -= 1;
        path[..length].copy_from_slice(&base[..length]);
        path[length..length + suffix.len()].copy_from_slice(suffix);
        return length + suffix.len();
    }
    let suffix = b"/wolfetch/config\0";
    if length + suffix.len() > path.len() {
        return 0;
    }
    length -= 1;
    path[..length].copy_from_slice(&base[..length]);
    path[length..length + suffix.len()].copy_from_slice(suffix);
    length + suffix.len()
}

fn apply_line(config: &mut Config, line: &[u8]) {
    let Some(split) = line.iter().position(|byte| *byte == b'=') else {
        return;
    };
    let key = trim(&line[..split]);
    let value = trim(&line[split + 1..]);
    if key == b"theme" {
        set_theme(config, value);
    } else if key == b"show" {
        set_fields(config, value);
    } else if key == b"wm" {
        config.wm_override.set(value);
        config.wm_override.trim();
    } else if key == b"logo" {
        config.logo = value != b"none";
    } else if key == b"show_runtime" {
        config.runtime = value != b"false";
    } else if key == b"show_process_memory" {
        config.process_memory = value != b"false";
    } else if key == b"color_art" {
        config.theme.art = number(value) as u8;
    } else if key == b"color_art_light" {
        config.theme.art_light = number(value) as u8;
    } else if key == b"color_shadow" {
        config.theme.shadow = number(value) as u8;
    } else if key == b"color_deep_shadow" {
        config.theme.deep_shadow = number(value) as u8;
    } else if key == b"color_label" {
        config.theme.label = number(value) as u8;
    } else if key == b"color_value" {
        config.theme.value = number(value) as u8;
    } else if key == b"color_stats" {
        config.theme.stats = number(value) as u8;
    }
}

fn set_theme(config: &mut Config, value: &[u8]) {
    config.theme = if value == b"mono" {
        Palette::mono()
    } else if value == b"ocean" {
        Palette::ocean()
    } else if value == b"gray" {
        Palette::gray()
    } else {
        Palette::royal()
    };
}

fn set_fields(config: &mut Config, value: &[u8]) {
    config.show = 0;
    config.order_len = 0;
    for item in value.split(|byte| *byte == b',') {
        let bit = match trim(item) {
            b"distro" | b"os" => FIELD_DISTRO,
            b"kernel" => FIELD_KERNEL,
            b"wm" => FIELD_WM,
            b"de" | b"desktop" => FIELD_DE,
            b"term" | b"terminal" => FIELD_TERM,
            b"shell" => FIELD_SHELL,
            b"cpu" => FIELD_CPU,
            b"gpu" => FIELD_GPU,
            b"memory" | b"ram" => FIELD_MEMORY,
            b"uptime" => FIELD_UPTIME,
            b"host" | b"hostname" => FIELD_HOST,
            b"load" | b"load_average" => FIELD_LOAD,
            b"disk" | b"disk_usage" => FIELD_DISK,
            b"resolution" | b"screen" => FIELD_RESOLUTION,
            b"board" | b"motherboard" => FIELD_BOARD,
            b"cpu_usage" | b"usage" => FIELD_CPU_USAGE,
            b"all" => 0,
            _ => 0,
        };
        if trim(item) == b"all" {
            config.show = 0;
            config.order_len = 0;
            for index in 0..16 {
                if FIELD_ALL & (1 << index) != 0 {
                    add_field(config, 1 << index);
                }
            }
        } else {
            add_field(config, bit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, apply_line, set_fields};
    use crate::model::{FIELD_CPU_USAGE, FIELD_DISK, FIELD_HOST};

    #[test]
    fn show_preserves_requested_order() {
        let mut config = Config::new();
        set_fields(&mut config, b"host,disk,cpu_usage");
        assert_eq!(config.order_len, 3);
        assert_eq!(config.order[0], 9);
        assert_eq!(config.order[1], 11);
        assert_eq!(config.order[2], 14);
        assert_eq!(config.show, FIELD_HOST | FIELD_DISK | FIELD_CPU_USAGE);
    }

    #[test]
    fn desktop_field_is_supported() {
        let mut config = Config::new();
        set_fields(&mut config, b"wm,desktop");
        assert_eq!(config.order_len, 2);
        assert_eq!(config.order[1], 15);
    }

    #[test]
    fn show_all_contains_every_field() {
        let mut config = Config::new();
        set_fields(&mut config, b"all");
        assert_eq!(config.order_len, 16);
        assert_eq!(config.show, crate::model::FIELD_ALL);
    }

    #[test]
    fn accepts_window_manager_override() {
        let mut config = Config::new();
        apply_line(&mut config, b"wm=dwl");
        assert_eq!(&config.wm_override.data[..config.wm_override.len], b"dwl");
    }
}

fn add_field(config: &mut Config, bit: u16) {
    if bit != 0 && config.order_len < config.order.len() && config.show & bit == 0 {
        config.show |= bit;
        config.order[config.order_len] = bit.trailing_zeros() as u8;
        config.order_len += 1;
    }
}

fn trim(value: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = value.len();
    while start < end && value[start].is_ascii_whitespace() {
        start += 1;
    }
    while end > start && value[end - 1].is_ascii_whitespace() {
        end -= 1;
    }
    &value[start..end]
}

fn number(value: &[u8]) -> u64 {
    let mut output = 0;
    for &byte in value {
        if byte.is_ascii_digit() {
            output = output * 10 + u64::from(byte - b'0');
        }
    }
    output
}
