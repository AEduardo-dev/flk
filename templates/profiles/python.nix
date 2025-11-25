{pkgs}: let
in {
  description = "Python development environment";

  packages = with pkgs; [
    poetry
    python311
    python311Packages.pip
    python311Packages.virtualenv
    black
    pyright
    mypy
    ruff
  ];

  envVars = {
    RUST_BACKTRACE = "1";
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

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
