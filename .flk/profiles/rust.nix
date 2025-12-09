{pkgs, ...}: {
  packages = [
    pkgs.rust-bin.stable.latest.default # From rust-overlay
    pkgs.rust-analyzer
    pkgs.pkg-config
    pkgs.cargo-watch
    pkgs.cargo-edit
    pkgs.cargo-dist
    pkgs."openssl@3.6.0"
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
}
