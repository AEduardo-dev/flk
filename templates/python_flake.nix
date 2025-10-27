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
        python = pkgs.python311;
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Python and tools
            poetry
            poetryPlugins
            python
            python311Packages.pip
            python311Packages.virtualenv

            # Additional tools
            black
            mypy
            ruff

            # User packages
          ];

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
        };
      }
    );
}
