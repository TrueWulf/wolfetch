<div align="center">
  <h1>wolfetch</h1>
  <p><strong>Ultra-fast. Minimal by design.</strong></p>
  <p>
    <a href="https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml"><img src="https://github.com/TrueWulf/wolfetch/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
    <a href="https://github.com/TrueWulf/wolfetch/releases"><img src="https://img.shields.io/github/v/release/TrueWulf/wolfetch?include_prereleases&sort=semver" alt="Latest release"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-GPLv3-blue.svg" alt="License"></a>
  </p>
  <img src="wolfetch-fetch.png" alt="wolfetch preview" width="720">
</div>

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
**9.6x faster than Fastfetch** and **35.2x faster than Macchina** in this test.
These are local results, not universal performance guarantees. Macchina's wiki
also reports **3.3 ms** on a different Intel i5-8265U Linux system using
Hyperfine; that historical result is not mixed into this comparison.

| Program | Median startup | Relative to wolfetch |
| --- | ---: | ---: |
| wolfetch | 0.768 ms | 1.0x |
| Fastfetch | 7.349 ms | 9.6x slower |
| Macchina | 27.032 ms | 35.2x slower |

Fastfetch and Macchina were installed for the local run. Neofetch, Screenfetch,
Pfetch, Ufetch, Hyfetch and Archey were also checked but were not installed in
the test environment, so no numbers are invented for them.

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
