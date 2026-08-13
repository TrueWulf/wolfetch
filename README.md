# wolfetch

> A tiny, fast system fetch for Linux and BSD.

wolfetch prints a clean system summary beside a minimal wolf. It is designed
for terminals, starts quickly, has no runtime daemon, does not require
systemd, and uses a small `key=value` configuration file.

## Preview

Screenshots will be added after the `0.5.0` layout is finalized.

<!-- screenshot: Linux terminal output -->
<!-- screenshot: BSD terminal output -->

## Features

- Native `no_std` executable with no third-party runtime dependencies.
- Linux information from `/proc`, `/sys`, libc and the environment.
- Configurable fields such as distro, kernel, host, CPU, GPU, memory, disk,
  load, resolution and motherboard.
- Plain text, JSON, fast and no-logo modes.
- One light royal-blue wolf with configurable ANSI colors.
- Works without systemd, OpenRC, runit, s6 or any background service.
- Separate platform backends for Linux and the BSD family.

## Quick Start

```sh
wolfetch
wolfetch --plain
wolfetch --json
wolfetch --fast
wolfetch --help
```

## Install

Release archives and packages will be attached to the `0.5.0-pre.1` GitHub
release. Distribution repositories may package a later version under their own
review process.

### Binary archive

Download the archive for your OS and architecture, then install it somewhere
on `PATH`:

```sh
install -Dm755 wolfetch ~/.local/bin/wolfetch
```

### Debian and Ubuntu

```sh
sudo apt install ./wolfetch_0.5.0-pre.1_amd64.deb
```

### Arch Linux

```sh
makepkg -si
```

After the package is available in AUR:

```sh
yay -S wolfetch
paru -S wolfetch
```

### Alpine, Void and Slackware

Packaging recipes are in `packaging/`. They can be used by Alpine
`abuild`, Void `xbps-src` and Slackware `makepkg` without systemd.

### BSD

FreeBSD, OpenBSD, NetBSD and DragonFly BSD packaging files are in
`packaging/bsd/`. Build from source with Cargo or use the matching release
archive while the ports are reviewed by each project.

### Build from source

```sh
cargo build --release
install -Dm755 target/release/wolfetch ~/.local/bin/wolfetch
```

## Configuration

The default configuration path is:

```text
${XDG_CONFIG_HOME}/wolfetch/config
~/.config/wolfetch/config
```

Use `--config PATH` to select another file. The format intentionally stays
small and readable:

```ini
theme=royal
show=distro,kernel,host,wm,term,shell,cpu,gpu,memory,disk,load,uptime
logo=wolf
show_runtime=true
show_process_memory=true
```

Available fields are `distro`, `kernel`, `host`, `wm`, `term`, `shell`,
`cpu`, `cpu_usage`, `gpu`, `memory`, `disk`, `load`, `resolution`, `board` and
`uptime`. `load` means load average; `cpu_usage` samples CPU activity.
The order in `show=` controls the order of the information lines.

See `docs/configuration.md` and `config.example` for all options.

## Platforms

| Platform | Status |
| --- | --- |
| Linux | Supported |
| FreeBSD | Backend in progress for 0.5.0-pre.1 |
| OpenBSD | Backend in progress for 0.5.0-pre.1 |
| NetBSD | Backend in progress for 0.5.0-pre.1 |
| DragonFly BSD | Backend in progress for 0.5.0-pre.1 |

The pre-release status is intentional until each BSD target has passed native
build and smoke tests.

## License

wolfetch is licensed under the GNU General Public License, version 3.
See `LICENSE`.

## Contributing

Bug reports, platform test results, packaging improvements and screenshots are
welcome. See `CONTRIBUTING.md` before opening a pull request.
