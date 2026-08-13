# Installation

wolfetch is an ultra-fast, minimal foreground CLI program. It does not install
or start a service and has no dependency on systemd, OpenRC, runit, s6 or any
other init system.

## Choose Your Method

| System | Recommended method | Status |
| --- | --- | --- |
| Arch Linux | `makepkg`, then AUR | `PKGBUILD` ready; AUR submission pending |
| Debian/Ubuntu | `.deb` release asset | Release package |
| Fedora/RHEL | RPM recipe or archive | Recipe included |
| Alpine | `abuild` | Recipe included |
| Void | `xbps-src` | Template included |
| Slackware | SlackBuild | Packaging notes included |
| FreeBSD | native port or archive | Port preparation |
| OpenBSD | native port or archive | Port preparation |
| NetBSD | native port or archive | Port preparation |
| DragonFly BSD | native port or archive | Port preparation |

For the most recent build, open
[Releases](https://github.com/TrueWulf/wolfetch/releases) and choose the archive
matching your OS and architecture.

## GitHub Releases

Every tagged release contains:

- Linux GNU archive;
- Linux musl archive;
- SHA256 checksums;
- source code archive;
- package files when the release build can produce them.

Install a downloaded binary into a user-local directory:

```sh
mkdir -p ~/.local/bin
install -m755 wolfetch ~/.local/bin/wolfetch
```

Make sure `~/.local/bin` is on your `PATH`.

## Package Families

- Debian/Ubuntu: install the `.deb` release asset with `apt`.
- Arch: clone this repository, enter `packaging/arch` and run `makepkg -si -f`; after AUR acceptance,
  use `yay -S wolfetch` or `paru -S wolfetch`.
- Fedora/RHEL: build or install the supplied RPM spec.
- Alpine: build with `abuild` from `packaging/alpine/APKBUILD`.
- Void: build with `xbps-src` from `packaging/void`.
- Slackware: use the SlackBuild under `packaging/slackware`.
- BSD: use the platform-specific port files under `packaging/bsd`.

Direct archives and checksums are the universal fallback for systems without a
repository package.

## Build From Source

```sh
git clone https://github.com/TrueWulf/wolfetch.git
cd wolfetch
cargo build --release
install -Dm755 target/release/wolfetch ~/.local/bin/wolfetch
```

The source build works on systems without systemd because wolfetch only runs
when called and reads local kernel/system interfaces directly.

## AUR Submission

The repository contains the AUR-ready file at
[`packaging/arch/PKGBUILD`](../packaging/arch/PKGBUILD). The first submission
must be pushed to an AUR repository named `wolfetch` by an account with AUR
SSH access. Until then, the GitHub clone plus `makepkg -si -f` is equivalent.
