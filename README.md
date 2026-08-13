# wolfetch

> Ultra-fast. Minimal by design. Built for Linux and BSD.

[![CI](https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/TrueWulf/wolfetch?include_prereleases&sort=semver)](https://github.com/TrueWulf/wolfetch/releases)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](LICENSE)

wolfetch prints a clean system summary beside a minimal wolf. Its main concept
is simple: collect only useful local information, start almost instantly, and
stay small enough to disappear into any terminal setup. There is no daemon,
no systemd dependency, no shell command execution and no complicated config
format.

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

Choose the method for your system in the dedicated
[Installation guide](docs/installation.md). The newest binaries and checksums
are always in [GitHub Releases](https://github.com/TrueWulf/wolfetch/releases).

### Fastest: binary archive

Download the archive for your OS and architecture, then install it somewhere
on `PATH`:

```sh
install -Dm755 wolfetch ~/.local/bin/wolfetch
```

### Debian and Ubuntu

```sh
sudo apt install ./wolfetch_0.5.0-pre.2_amd64.deb
```

### Arch Linux and AUR

```sh
git clone https://github.com/TrueWulf/wolfetch.git
cd wolfetch
cd packaging/arch
makepkg -si -f
```

The `PKGBUILD` is ready for AUR submission. Once the package is accepted by
the AUR, install it with:

```sh
yay -S wolfetch
paru -S wolfetch
```

### Alpine, Void and Slackware

Packaging recipes are in `packaging/`. They are documented in the
[Installation guide](docs/installation.md) and do not require systemd.

### BSD

FreeBSD, OpenBSD, NetBSD and DragonFly BSD are release targets. Use the
matching archive or build from source while native ports are reviewed.

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

The small gray line at the bottom shows wolfetch's own startup time and
process memory. It is not the machine's total memory usage; that is the
`Memory` field above.

## Project Status

`0.5.0-pre.2` is the first public pre-release with both command names. Linux is the primary tested
platform. BSD backends and package recipes are included for native testing and
will be promoted to stable support after their CI and smoke tests pass.

## Platforms

| Platform | Status |
| --- | --- |
| Linux | Supported |
| FreeBSD | Backend in progress for 0.5.0-pre.2 |
| OpenBSD | Backend in progress for 0.5.0-pre.2 |
| NetBSD | Backend in progress for 0.5.0-pre.2 |
| DragonFly BSD | Backend in progress for 0.5.0-pre.2 |

The pre-release status is intentional until each BSD target has passed native
build and smoke tests.

## License

wolfetch is licensed under the GNU General Public License, version 3.
See `LICENSE`.

## Contributing

Bug reports, platform test results, packaging improvements and screenshots are
welcome. See `CONTRIBUTING.md` before opening a pull request.
