{pkgs, ...}: {
  packages = [
    (pkgs.rust-bin.stable.latest.default.override {
      extensions = [
        "clippy"
        "llvm-tools-preview"
        "rust-src"
        "rust-analyzer"
      ];
    })
    pkgs.pkg-config
    pkgs.openssl
    pkgs.cargo-watch
    pkgs.cargo-edit
    pkgs.cargo-dist
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
