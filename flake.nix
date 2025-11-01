{
  description = "Rust development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        devPackages = with pkgs; [
          # Rust toolchain
          rust-bin.stable.latest.default
          rust-analyzer
          rustup

          # Build tools
          pkg-config
          openssl

          # Additional tools
          cargo-watch
          cargo-edit
          cargo-dist
          release-plz

          # User packages
        ];
        containerPackages = with pkgs; [
          bashInteractive
          coreutils
          findutils
          gnugrep
          git
        ];
        devEnv = {
          LANG = "en_US.UTF-8";
          LC_ALL = "en_US.UTF-8";
          RUST_BACKTRACE = "1";
          TEST = "works";
        };
        shellHook = ''
          echo "🦀 Rust development environment ready!"
          echo "Rust version: $(rustc --version)"
          source .flk/hooks.sh

          # Custom commands will be added here
        '';
      in {
        devShells.default = pkgs.mkShell ({
            packages = devPackages;
            shellHook = shellHook;
          }
          // devEnv);
        packages.docker = pkgs.dockerTools.buildLayeredImage {
          name = "rust-dev";
          tag = "latest";
          contents = devPackages ++ containerPackages;
          config = {
            Cmd = ["${pkgs.bashInteractive}/bin/bash"];
            Env = pkgs.lib.mapAttrsToList (name: value: "${name}=${value}") devEnv;
            WorkingDir = "/workspace";
          };
        };
      }
    );
}
