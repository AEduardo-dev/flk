{pkgs, ...}: {
  packages = [
    pkgs.rust-bin.stable.latest.default # From rust-overlay
    pkgs.rust-analyzer
    pkgs.pkg-config
    pkgs.openssl
    pkgs.cargo-watch
    pkgs.cargo-edit
    pkgs.cargo-dist
    pkgs.mdbook
  ];

  envVars = {
    RUST_BACKTRACE = "1";
  };

  commands = [
  ];

  shellHook = ''
    echo "🦀 Rust development environment ready!"
    echo "Rust version: $(rustc --version)"
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
