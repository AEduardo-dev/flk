{pkgs, ...}: let
in {
  description = "Python development environment";

  packages = [
    pkgs.poetry
    pkgs.python311
    pkgs.python311Packages.pip
    pkgs.python311Packages.virtualenv
    pkgs.black
    pkgs.pyright
    pkgs.mypy
    pkgs.ruff
  ];

  envVars = {
    RUST_BACKTRACE = "1";
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

  commands = [];

  shellHook = ''
    echo "🐍 Python development environment ready!"
    echo "Python version: $(python --version)"

    # Create virtual environment if it doesn't exist
    if [ ! -d .venv ]; then
      python -m venv .venv
    fi

    source .venv/bin/activate

    # Custom commands will be added here
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
