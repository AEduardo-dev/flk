{
  description = "Rust development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    profile-lib.url = "path:/home/angel/Documents/repos/nix-profile-lib";
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
    profile-lib,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      profileLib = profile-lib.lib {inherit pkgs;};

      profileDefinitions = {
        base = {
          packages = with pkgs; [
            bashInteractive
            coreutils
            findutils
            gnugrep
            git
          ];

          envVars = {
            LANG = "en_US.UTF-8";
            LC_ALL = "en_US.UTF-8";
          };

          shellHook = ''
            echo "✓ Base tools loaded"
          '';

          containerConfig = {
            Cmd = ["${pkgs.bashInteractive}/bin/bash"];
            WorkingDir = "/workspace";
          };
        };

        rust = {
          packages = with pkgs; [
            rust-bin.stable.latest.default
            rust-analyzer
            rustup
            pkg-config
            openssl
            cargo-watch
            cargo-edit
            cargo-dist
            release-plz
          ];

          envVars = {
            RUST_BACKTRACE = "1";
          };

          shellHook = ''
            echo "🦀 Rust development environment ready!"
            echo "Rust version: $(rustc --version)"
          '';

          containerConfig = {
            Cmd = ["${pkgs.bashInteractive}/bin/bash"];
          };
        };
      };
    in
      profileLib.mkProfileOutputs {
        inherit profileDefinitions;
        defaultImage = "base-rust";
      });
}
