# Startup Benchmark

This is a local startup measurement, not a universal performance claim. Each
program is launched as a fresh process with output redirected to `/dev/null`.
Hyperfine performs five warmups and 50 measured runs with shell startup
disabled (`--shell=none`). This compares default data collection; wolfetch is
run with `--plain`, pfetch-rs with its default command, Fastfetch with
`--pipe`, and Macchina with its default command. wolfetch's CPU usage read is
included in the startup measurement.

## Environment

- Host: AMD Ryzen 7 5700X, 8 cores
- OS: Arch Linux
- wolfetch: `0.5.4`, built from this checkout with `cargo build --release`
- Fastfetch: `2.66.0`
- Macchina: `6.4.0`
- Timer: Hyperfine `1.20.0`, `--warmup 5 --runs 50 --shell=none`
- Output: redirected to `/dev/null`

## Measured Run

Command:

```sh
bash benchmarks/startup.sh
```

The benchmark should be rerun after hardware, operating-system, or package
changes. Numbers below are from one local run and should not be presented as
representative of every system.

| Program | Mean | Minimum | Maximum |
| --- | ---: | ---: | ---: |
| wolfetch | 650.6 us | 575.7 us | 808.6 us |
| pfetch-rs | 2.6 ms | 2.2 ms | 3.1 ms |
| Fastfetch | 6.6 ms | 5.5 ms | 7.9 ms |
| Macchina | 25.9 ms | 23.5 ms | 28.5 ms |

## External Reference

Macchina's historical benchmark page reports 3.3 ms on Linux with an Intel
Core i5-8265U and Hyperfine flags `-w 4 -m 500 -N`. It is included for context
only: the hardware, software versions, flags and date differ from this run.
