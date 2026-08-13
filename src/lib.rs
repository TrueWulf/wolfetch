#![no_std]

pub mod args;
pub mod art;
pub mod config;
pub mod model;
pub mod output;
pub mod platform;

pub fn run(argc: i32, argv: *const *const u8) -> i32 {
    let args = args::parse(argc, argv);
    if args.error {
        output::error();
        return 2;
    }
    if args.help {
        output::help();
        return 0;
    }
    if args.version {
        output::version();
        return 0;
    }
    let mut config = model::Config::new();
    config::load(&mut config, args.config);
    config::theme_arg(&mut config, args.theme);
    if args.no_logo {
        config.logo = false;
    }
    if args.fast {
        config.show &= !(model::FIELD_CPU | model::FIELD_CPU_USAGE | model::FIELD_GPU);
        config.process_memory = false;
    }
    let start = platform::now();
    let info = platform::collect(&config, start);
    output::render(&info, &config, args.plain, args.no_logo, args.json);
    0
}
