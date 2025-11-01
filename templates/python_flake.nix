{
  description = "Python development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
        devPackages = with pkgs; [
          # Python and tools
          poetry
          python313
          python313Packages.pip
          python313Packages.virtualenv

          # Additional tools
          black
          pyright
          mypy
          ruff

          # User packages
        ];
        containerPackages = with pkgs; [
          bashInteractive
          coreutils
          findutils
          gnugrep
          git
        ];
        shellHook = ''
          echo "🐍 Python development environment ready!"
          echo "Python version: $(python --version)"
          source .flk/hooks.sh

          # Create virtual environment if it doesn't exist
          if [ ! -d .venv ]; then
            python -m venv .venv
          fi
          source .venv/bin/activate

          # Custom commands will be added here
        '';
        devEnv = {
          LANG = "en_US.UTF-8";
          LC_ALL = "en_US.UTF-8";
        };
      in {
        devShells.default = pkgs.mkShell ({
            packages = devPackages;
            shellHook = shellHook;
          }
          // devEnv);
        packages.docker = pkgs.dockerTools.buildLayeredImage {
          name = "python-dev";
          tag = "latest";
          contents = devPackages ++ containerPackages;
          config = {
            Cmd = ["${pkgs.bashInteractive}/bin/bash"];
            Env = pkgs.lib.mapAttrsToList (name: value: "${name}=${value}") devEnv;
            WorkingDir = "/workspace";
          };
        };
      }
    );
}
