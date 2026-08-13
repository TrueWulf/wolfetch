# wolfetch

> Ultra-fast. Minimal by design.

<p align="center">
  <a href="https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml"><img src="https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/TrueWulf/wolfetch/releases"><img src="https://img.shields.io/github/v/release/TrueWulf/wolfetch?include_prereleases&sort=semver" alt="Latest release"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPLv3-blue.svg" alt="License"></a>
</p>

<p align="center">
  <img src="wolfetch-fetch.png" alt="wolfetch preview" width="720">
</p>

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

## Benchmark

Measured locally on Arch Linux with 15 warm launches, wolfetch started about
**8.6x faster than Fastfetch** and **33.7x faster than Macchina** in this test.
These are local results, not universal performance guarantees.

| Program | Median startup | Relative to wolfetch |
| --- | ---: | ---: |
| wolfetch | 0.783 ms | 1.0x |
| Fastfetch | 6.737 ms | 8.6x slower |
| Macchina | 26.362 ms | 33.7x slower |

Run the reproducible benchmark yourself:

```sh
bash benchmarks/startup.sh
```

See the recorded run and test conditions in
[`benchmarks/results.md`](benchmarks/results.md).

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
