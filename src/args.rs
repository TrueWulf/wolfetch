use crate::model::Args;
use core::ptr;

pub fn parse(argc: i32, argv: *const *const u8) -> Args {
    let mut args = Args {
        plain: false,
        json: false,
        no_logo: false,
        fast: false,
        theme: ptr::null(),
        config: ptr::null(),
        help: false,
        version: false,
        error: false,
    };
    let mut index = 1isize;
    while index < argc as isize {
        let arg = unsafe { *argv.offset(index) };
        if equals(arg, b"--help") || equals(arg, b"-h") {
            args.help = true;
        } else if equals(arg, b"--version") || equals(arg, b"-V") {
            args.version = true;
        } else if equals(arg, b"--plain") || equals(arg, b"-p") {
            args.plain = true;
        } else if equals(arg, b"--json") || equals(arg, b"-j") {
            args.json = true;
        } else if equals(arg, b"--no-logo") || equals(arg, b"-n") {
            args.no_logo = true;
        } else if equals(arg, b"--fast") || equals(arg, b"-f") {
            args.fast = true;
        } else if equals(arg, b"--theme") {
            index += 1;
            if index >= argc as isize {
                args.error = true;
                break;
            }
            args.theme = unsafe { *argv.offset(index) };
        } else if equals(arg, b"--config") {
            index += 1;
            if index >= argc as isize {
                args.error = true;
                break;
            }
            args.config = unsafe { *argv.offset(index) };
        } else {
            args.error = true;
            break;
        }
        index += 1;
    }
    args
}

fn equals(pointer: *const u8, text: &[u8]) -> bool {
    let mut index = 0;
    while index < text.len() {
        if unsafe { *pointer.add(index) } != text[index] {
            return false;
        }
        index += 1;
    }
    unsafe { *pointer.add(index) == 0 }
}
