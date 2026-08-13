# Platform Support

Linux is the primary platform for `0.5.1`. Its backend reads stable local
interfaces and works independently of the init system.

The BSD backends are separated by target so platform-specific system APIs do
not leak into the Linux implementation. FreeBSD, OpenBSD, NetBSD and DragonFly
BSD are release targets. FreeBSD has a VM CI job; the other BSD targets remain
pre-release until native CI jobs and smoke tests are available.

Supported data is allowed to vary by platform. A field that is unavailable on
a particular kernel should display `Unknown`, not fail the whole fetch.

## Linux Detection Scope

- CPU: reads the kernel model name from `/proc/cpuinfo`.
- GPU: scans DRM cards and uses NVIDIA information plus DRM driver and device
  metadata for NVIDIA, AMD and Intel adapters. Multiple adapters are currently
  reported as the first detected adapter.
- WM: detects Hyprland, Sway, i3, bspwm, Wayland and X11 using session
  environment markers.
- DE: reads `XDG_CURRENT_DESKTOP`, `XDG_SESSION_DESKTOP` or
  `DESKTOP_SESSION`; compositor names such as Hyprland and Sway remain `DE:
  Unknown` because they are not desktop environments.
- Resolution: scans common DRM connectors and reports the first available mode.
  Wayland scale and fractional logical resolution are not inferred from the
  compositor.

Detection is intentionally dependency-free. It does not invoke shell commands,
desktop-specific tools or GPU vendor utilities.
