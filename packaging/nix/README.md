# NixOS

The repository includes a flake entry point so users can run wolfetch without
installing a system service:

```sh
nix run github:TrueWulf/wolfetch
nix profile install github:TrueWulf/wolfetch
```

The package exports both `wolfetch` and `wfetch`.
