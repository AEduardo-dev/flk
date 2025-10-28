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
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
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
          ];

          shellHook = ''
            echo "🦀 Rust development environment ready!"
            echo "Rust version: $(rustc --version)"

            # Custom commands will be added here
          '';

          # Environment variables
          RUST_BACKTRACE = "1";
        };
      }
    );
}
