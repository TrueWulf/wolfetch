# wolfetch

> Ultra-fast. Minimal by design.

[![CI](https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/TrueWulf/wolfetch?include_prereleases&sort=semver)](https://github.com/TrueWulf/wolfetch/releases)
[![License](https://img.shields.io/badge/license-GPLv3-blue.svg)](LICENSE)

![wolfetch preview](wolfetch-fetch.png)

wolfetch is a small command-line system summary with a minimal wolf beside it.
It starts almost instantly, stays easy to understand, and gives you useful
information without daemons, background services, or a complicated config
format.

## Quick Start

```sh
wolfetch
wfetch
wolfetch --plain
wolfetch --json
```

## Features

- Ultra-fast startup and a small native binary.
- Minimal royal-blue terminal output.
- Configurable fields and field order.
- JSON, plain, fast and no-logo modes.
- Simple `key=value` configuration.
- No system service or runtime daemon.
- No shell commands executed from configuration.

## Installation

Use the [Installation guide](docs/installation.md) for package-manager
commands, release archives and building from source.

The latest downloads are available in
[GitHub Releases](https://github.com/TrueWulf/wolfetch/releases).

## Configuration

```ini
theme=royal
show=distro,kernel,host,wm,term,shell,cpu,gpu,memory,disk,load,uptime
logo=wolf
show_runtime=true
show_process_memory=true
```

See [configuration.md](docs/configuration.md) for all fields and options.

## Support

Linux is the primary tested platform. FreeBSD, OpenBSD, NetBSD and DragonFly
BSD are active pre-release targets. See the
[platform support status](docs/platform-support.md) for details.

## License

GPLv3. See [LICENSE](LICENSE).
