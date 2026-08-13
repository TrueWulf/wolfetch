# Startup Benchmark

This is a local startup measurement, not a universal performance claim. Each
program is launched as a fresh process with output redirected to `/dev/null`.
Hyperfine 1.20.0 performs four warmups and 30 measured runs with shell startup
disabled (`--shell=none`). The commands use their default output and options.

## Environment

- Host: AMD Ryzen 7 5700X, 8 cores
- OS: Arch Linux
- wolfetch: `0.5.0`, built from this checkout with `cargo build --release`
- Fastfetch: `2.66.0`
- Macchina: `6.4.0`
- Timer: Hyperfine `1.20.0`, `--warmup 4 --runs 30 --shell=none`
- Output: redirected to `/dev/null`

## Measured Run

Command:

```sh
bash benchmarks/startup.sh 30 4
```

The benchmark should be rerun after hardware, operating-system, or package
changes. Numbers below are from one local run and should not be presented as
representative of every system.

| Program | Mean | Minimum | Maximum |
| --- | ---: | ---: | ---: |
| wolfetch | 0.506 ms | 0.408 ms | 0.600 ms |
| Fastfetch | 6.5 ms | 5.1 ms | 7.6 ms |
| Macchina | 25.7 ms | 24.3 ms | 28.1 ms |
| pfetch | 2.6 ms | 2.3 ms | 3.0 ms |
| screenfetch | 1.097 s | 1.039 s | 1.179 s |

## External Reference

Macchina's historical benchmark page reports 3.3 ms on Linux with an Intel
Core i5-8265U and Hyperfine flags `-w 4 -m 500 -N`. It is included for context
only: the hardware, software versions, flags and date differ from this run.

## Other Fetch Commands Checked

The following commands were checked but not included in the measured table:
`neofetch`, `ufetch`, and `archey` were not installed. `hyfetch` requires an
interactive first-run configuration in this environment, so it is excluded
until a fixed non-interactive configuration is available.
