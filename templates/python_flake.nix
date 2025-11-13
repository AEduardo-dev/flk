{
  description = "Python development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    profile-lib.url = "github:AEduardo-dev/nix-profile-lib";
  };

  outputs = {
    self,
    flake-utils,
    nixpkgs,
    profile-lib,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        profileLib = profile-lib.lib {inherit pkgs;};

        profileDefinitions = {
          python = {
            packages = with pkgs; [
              poetry
              python312
              python312Packages.pip
              python312Packages.virtualenv
              black
              pyright
              mypy
              ruff
            ];

            envVars = {
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
          };
        };
      in
        profileLib.mkProfileOutputs {
          inherit profileDefinitions;
          defaultShell = "python";
          defaultImage = "python";
        }
    );
}
