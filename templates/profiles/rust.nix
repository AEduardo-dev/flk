{
  pkgs,
  system,
}: {
  packages = with pkgs; [
    rust-bin.stable.latest.default # From rust-overlay
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
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
