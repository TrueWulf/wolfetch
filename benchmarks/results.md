# Startup Benchmark

This is a local startup measurement, not a universal performance claim. Each
program is launched as a fresh process with output redirected to `/dev/null`.
Hyperfine performs five warmups and 50 measured runs with shell startup
disabled (`--shell=none`). This compares default data collection; wolfetch is
run with `--plain`, plus an optional `--gpu-usage` run, pfetch-rs with its
default command, Fastfetch with `--pipe`, and Macchina with its default
command. wolfetch's CPU usage read is included in the startup measurement.
The optional NVML path is intentionally not part of the default startup claim.

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
| wolfetch default | 675.6 us | 548.4 us | 983.8 us |
| wolfetch `--gpu-usage` | 25.3 ms | 23.0 ms | 29.2 ms |
| pfetch-rs | 3.0 ms | 2.5 ms | 3.8 ms |
| Fastfetch | 8.3 ms | 7.0 ms | 9.9 ms |
| Macchina | 32.4 ms | 29.2 ms | 40.6 ms |

## External Reference

Macchina's historical benchmark page reports 3.3 ms on Linux with an Intel
Core i5-8265U and Hyperfine flags `-w 4 -m 500 -N`. It is included for context
only: the hardware, software versions, flags and date differ from this run.

The default path is the fast comparison. `--gpu-usage` dynamically loads NVML
when available and is slower by design; its timing is reported separately.
