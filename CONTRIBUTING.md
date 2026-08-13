# Contributing

## Development

Run the checks before opening a change:

```sh
cargo fmt --check
cargo check
cargo test
cargo build --release
```

Platform changes should include the target triple, operating system version,
terminal and a plain-text sample of the output.

## Scope

wolfetch avoids runtime daemons, shell command execution and unnecessary
dependencies. New fields should use native platform APIs or stable system
interfaces and should degrade to `Unknown` when unavailable.

Screenshots should be captured from a real terminal after the output layout is
stable. Do not include hostnames, usernames, IP addresses or other private
information without redacting them.
