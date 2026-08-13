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

The field names are `distro`, `kernel`, `host`, `wm`, `de`, `term`, `shell`,
`cpu`, `cpu_usage`, `gpu`, `memory`, `disk`, `load`, `resolution`, `board` and
`uptime`. `wm` is the compositor/window manager; `de` is the desktop
environment when the session exposes one. `load` means load average;
`cpu_usage` samples CPU activity.

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
