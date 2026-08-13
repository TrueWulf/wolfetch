# Startup Benchmark

This is a local startup measurement, not a universal performance claim. Each
program is launched as a fresh process, its output is redirected to `/dev/null`,
and the median, minimum and maximum are reported over 15 warm runs. The timing
includes process startup and the program's default data collection.

## Environment

- Host: AMD Ryzen 7 5700X, 8 cores
- OS: Arch Linux
- wolfetch: `0.5.0-pre.7`, built from this checkout with `cargo build --release`
- Fastfetch: `2.66.0`
- Macchina: `6.4.0`
- Timer: Bash `EPOCHREALTIME`, microsecond wall-clock timestamps with `LC_ALL=C`
- Output: redirected to `/dev/null`

## Measured Run

Command:

```sh
bash benchmarks/startup.sh 15
```

The benchmark should be rerun after hardware, operating-system, or package
changes. Numbers below are from one local run and should not be presented as
representative of every system.

| Program | Median | Minimum | Maximum |
| --- | ---: | ---: | ---: |
| wolfetch | 0.783 ms | 0.657 ms | 0.861 ms |
| Fastfetch | 6.737 ms | 6.027 ms | 7.852 ms |
| Macchina | 26.362 ms | 25.185 ms | 32.423 ms |
