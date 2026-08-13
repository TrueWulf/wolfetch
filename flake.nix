{
  description = "Ultra-fast, minimal system fetch for Linux and BSD";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in {
      packages = forAllSystems (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "wolfetch";
          version = "0.5.0-pre.4";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          postInstall = ''
            ln -s $out/bin/wolfetch $out/bin/wfetch
          '';
          meta = {
            description = "Ultra-fast, minimal system fetch for Linux and BSD";
            homepage = "https://github.com/TrueWulf/wolfetch";
            license = pkgs.lib.licenses.gpl3Only;
            mainProgram = "wolfetch";
          };
        };
      });
    };
}
