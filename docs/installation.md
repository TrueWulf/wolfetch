# Installation

wolfetch is a foreground command. It does not install a daemon and does not
depend on systemd, OpenRC, runit, s6, or another init system.

## Arch Linux

### AUR

After the package is published in AUR:

```sh
yay -S wolfetch
```

or:

```sh
paru -S wolfetch
```

The AUR package is a separate Git repository containing `PKGBUILD` and
`.SRCINFO`. Its recipe is prepared in
[`packaging/arch`](../packaging/arch/).

### Build now from GitHub

Until AUR publication, install directly from this repository:

```sh
git clone https://github.com/TrueWulf/wolfetch.git
cd wolfetch/packaging/arch
makepkg -si -f
```

This installs both commands:

```sh
wolfetch
wfetch
```

### Prebuilt Arch package

Download the `.pkg.tar.zst` file from the latest
[GitHub Release](https://github.com/TrueWulf/wolfetch/releases), then run:

```sh
sudo pacman -U wolfetch-*.pkg.tar.zst
```

## Debian and Ubuntu

Download the `.deb` package from Releases and install it with:

```sh
sudo apt install ./wolfetch-*.deb
```

Direct `.deb` installation works immediately. `apt install wolfetch` will be
available after a Debian repository, PPA or Launchpad package is accepted.

## Fedora and RHEL

Build the RPM recipe from
[`packaging/rpm`](../packaging/rpm/) or install a future RPM release asset:

```sh
sudo dnf install ./wolfetch-*.rpm
```

Short command `dnf install wolfetch` requires a COPR or another RPM
repository.

## Alpine

The `APKBUILD` is in [`packaging/alpine`](../packaging/alpine/). Build it with
Alpine's `abuild`, or use a future Alpine repository package:

```sh
apk add wolfetch
```

The short command becomes available after the package is accepted into Alpine
`aports` and reaches a repository.

## Void Linux

The XBPS template is in [`packaging/void`](../packaging/void/). After it is
accepted into `void-packages`:

```sh
sudo xbps-install wolfetch
```

Until then, build the template with `xbps-src` or use a release archive.

## Gentoo

The ebuild is in
[`packaging/gentoo`](../packaging/gentoo/app-misc/wolfetch/). Install from a
local overlay with:

```sh
sudo emerge --ask app-misc/wolfetch
```

The short command works after the ebuild is accepted into the Gentoo tree or a
public overlay.

## NixOS

Run without installing system-wide:

```sh
nix run github:TrueWulf/wolfetch
```

Install into the user profile:

```sh
nix profile install github:TrueWulf/wolfetch
```

The flake exports both `wolfetch` and `wfetch`. A NixOS module or `nixpkgs`
package requires a separate review upstream.

## Slackware

Use the SlackBuild/package notes in
[`packaging/slackware`](../packaging/slackware/), or install from a release
archive. A short `slackpkg install wolfetch` command requires publication in a
Slackware repository.

## BSD

Release archives and source builds are available for BSD targets as they pass
native CI. Port preparation files are in
[`packaging/bsd`](../packaging/bsd/).

The intended short commands after native port review are:

```sh
pkg install wolfetch       # FreeBSD
pkg_add wolfetch           # OpenBSD
pkgin install wolfetch     # NetBSD
pkg install wolfetch       # DragonFly BSD
```

## Binary Archive

Download the archive matching the operating system and architecture from
[Releases](https://github.com/TrueWulf/wolfetch/releases), verify it, and
install both binaries:

```sh
sha256sum -c SHA256SUMS
mkdir -p ~/.local/bin
install -m755 wolfetch ~/.local/bin/wolfetch
install -m755 wfetch ~/.local/bin/wfetch
```

Make sure `~/.local/bin` is on `PATH`.

## Build From Source

```sh
git clone https://github.com/TrueWulf/wolfetch.git
cd wolfetch
cargo build --release
install -Dm755 target/release/wolfetch ~/.local/bin/wolfetch
install -Dm755 target/release/wfetch ~/.local/bin/wfetch
```

## Package Status

| Ecosystem | Current status |
| --- | --- |
| Arch/AUR | `PKGBUILD` and `.SRCINFO` prepared; AUR publication pending |
| Debian/Ubuntu | Release package path documented; repository publication pending |
| Fedora/RHEL | RPM recipe prepared; COPR/repository publication pending |
| Alpine | `APKBUILD` prepared; `aports` review pending |
| Void | XBPS template prepared; `void-packages` review pending |
| Gentoo | Ebuild prepared; tree/overlay publication pending |
| NixOS | Flake available from GitHub |
| Slackware | Packaging notes prepared |
| BSD | Port preparation and native review pending |
