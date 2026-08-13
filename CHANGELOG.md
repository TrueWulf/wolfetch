# Changelog

## 0.5.2

- Add CPU usage to CPU output and dependency-free GPU usage where Linux sysfs exposes it.
- Improve multi-GPU detection and preserve GPU uevent metadata while reading model names.
- Refresh the local benchmark definition and preview image.

## 0.5.1

- Added `--minimal` output mode.
- Added separate desktop environment reporting.
- Improved Linux GPU and display mode detection.
- Added output truncation warnings and regression coverage.

## 0.5.0

- First stable release.
- Includes Linux and BSD backends, both `wolfetch` and `wfetch` commands,
  configuration, JSON output, man pages and shell completions.

## 0.5.0-pre.7

- Fixed Debian asset naming so downloaded checksums match the published file.

## 0.5.0-pre.6

- Fixed release checksum paths so downloaded assets verify directly.

## 0.5.0-pre.5

- Fixed Debian release version conversion for prerelease package assets.

## 0.5.0-pre.4

- Added a release workflow for Debian packages.
- Kept the homepage minimal and moved install commands into `Installation`.
- Added the preview image and AUR metadata to the repository.

## 0.5.0-pre.3

- Simplified the repository homepage and moved installation details into one guide.
- Added the project preview image and AUR `.SRCINFO` metadata.
- Added a Debian package to release assets.

## 0.5.0-pre.2

- Install both `wolfetch` and `wfetch` from package builds and release archives.
- Clarify the meaning of final runtime and process-memory statistics.
- Add NixOS and Gentoo packaging entry points.

## 0.5.0-pre.1

- Refreshed the wolf layout so runtime statistics follow uptime directly.
- Unified the wolf artwork under a light royal-blue color.
- Added configurable host, load, disk, resolution and motherboard fields.
- Added Linux `statvfs`, hostname and load collection.
- Added initial BSD platform layout and packaging locations.
- Added GPLv3 licensing and project documentation.
