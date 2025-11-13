{
  description = "Rust development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    profile-lib.url = "github:AEduardo-dev/nix-profile-lib";
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
        defaultShell = "rust";
        defaultImage = "rust";
      });
}
