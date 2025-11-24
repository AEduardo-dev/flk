{
  pkgs,
  system,
}: let
  # Profile fetches the overlay itself
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";

  pkgsWithRust = import pkgs.path {
    inherit system;
    overlays = [(import rust-overlay)];
  };
in {
  packages = with pkgsWithRust; [
    rust-bin.stable.latest.default
    rust-analyzer
    pkg-config
    openssl
    cargo-watch
    cargo-edit
    cargo-dist
  ];

  envVars = {
    RUST_BACKTRACE = "1";
  };

  shellHook = ''
    echo "🦀 Rust development environment ready!"
    echo "Rust version: $(rustc --version)"
  '';

  containerConfig = {
    Cmd = ["${pkgsWithRust.bashInteractive}/bin/bash"];
  };
}
