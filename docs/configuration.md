# Configuration

wolfetch uses a line-oriented `key=value` file. Unknown lines are ignored so
the file stays forward-compatible, while values remain easy to edit in any
text editor.

## Location

wolfetch checks `${XDG_CONFIG_HOME}/wolfetch/config`, then
`~/.config/wolfetch/config`. An explicit `--config PATH` takes precedence.

## Options

| Key | Values | Default |
| --- | --- | --- |
| `theme` | `royal`, `mono`, `ocean`, `gray` | `royal` |
| `show` | comma-separated field names | default fields |
| `logo` | `wolf`, `none` | `wolf` |
| `show_runtime` | `true`, `false` | `true` |
| `show_process_memory` | `true`, `false` | `true` |
| `wm` | window manager name | auto-detected |

The field names are `distro`, `kernel`, `host`, `wm`, `de`, `term`, `shell`,
`cpu`, `cpu_usage`, `gpu`, `memory`, `disk`, `load`, `resolution`, `board` and
`uptime`. `wm` is the compositor/window manager; `de` is the desktop
environment when the session exposes one. `load` means load average;
`cpu_usage` reports the cumulative system CPU activity from `/proc/stat`. The
default `CPU` and `GPU` values include
usage in parentheses, for example `CPU: AMD Ryzen 7 (7%)` and
`GPU: NVIDIA GeForce RTX (N/A)`. GPU usage is reported only when a supported
Linux sysfs metric is available.

Set `wm=dwl` (or another name) when a window manager does not expose a stable
session marker. The override is explicit rather than guessing from unrelated
processes.

## Colors

Colors use the 256-color ANSI palette:

```ini
color_art=27
color_art_light=75
color_shadow=243
color_deep_shadow=238
color_label=75
color_value=255
color_stats=243
```

The wolf is rendered as one light royal-blue color in the default theme.

## Final Statistics

The final line contains two process statistics:

```text
0.14 ms | 1.56 MiB
```

The first value is wolfetch startup and collection time. The second value is
the resident memory used by wolfetch itself. It is not total system memory;
that is shown by the `Memory` field.
