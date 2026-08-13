# Installation

wolfetch is a normal foreground CLI program. It does not install or start a
service and has no dependency on systemd.

## Package families

- Debian/Ubuntu: install the `.deb` release asset with `apt`.
- Arch: use the supplied `PKGBUILD`; AUR helpers use `yay -S wolfetch` or
  `paru -S wolfetch` after the recipe is published.
- Fedora/RHEL: build or install the supplied RPM spec.
- Alpine: build with `abuild` from `packaging/alpine/APKBUILD`.
- Void: build with `xbps-src` from `packaging/void`.
- Slackware: use the SlackBuild under `packaging/slackware`.
- BSD: use the platform-specific port files under `packaging/bsd`.

Direct archives and checksums are the universal fallback for systems without a
repository package.
