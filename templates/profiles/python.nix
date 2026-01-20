{pkgs, ...}: let
in {
  description = "Python development environment";

  packages = [
    pkgs.python3
    pkgs.virtualenv
    pkgs.black
    pkgs.pyright
    pkgs.mypy
    pkgs.ruff
  ];

  envVars = {
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

  shellHook = ''
    echo "🐍 Python development environment ready!"
    echo "Python version: $(python --version)"

    # Upgrade pip & install poetry into the venv
    # NOTE: This does not install poetry globally, only within the devshell environment
    # and it is done due to nixpkgs poetry versions being mismatched with python versions.
    pip install --upgrade pip
    pip install poetry


    # Check if poetry commands are available
    if ! command -v poetry &> /dev/null; then
      echo "Poetry could not be found. Using virtualenv instead."
      # Create virtual environment if it doesn't exist, then activate
      if [ ! -d .venv ]; then
        python -m venv .venv
        source .venv/bin/activate
        pip install -r requirements.txt
      else
        source .venv/bin/activate
      fi

    else
      # Create virtual environment if it doesn't exist, then activate
      if [ ! -d .venv ]; then
        poetry install
        $(poetry env activate)

      else
        source .venv/bin/activate
      fi

    fi


  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
