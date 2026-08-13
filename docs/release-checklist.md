# Release Checklist

## Before tagging

- Update `VERSION` in `src/model.rs`.
- Update `Cargo.toml` and `Cargo.lock`.
- Update `CHANGELOG.md`.
- Run formatting, checks and tests.
- Build every target listed as supported in the README.
- Run `--help`, `--version`, `--plain`, `--json` and `--fast`.
- Redact private information from screenshots.
- Generate SHA256 checksums for archives.

## Release assets

Use names that identify the project, version, target and archive format:

```text
wolfetch-0.5.0-pre.1-linux-x86_64-gnu.tar.gz
wolfetch-0.5.0-pre.1-linux-x86_64-musl.tar.gz
wolfetch-0.5.0-pre.1-freebsd-x86_64.tar.gz
wolfetch-0.5.0-pre.1-openbsd-x86_64.tar.gz
```
