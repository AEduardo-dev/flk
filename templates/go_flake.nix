{
  description = "Go development environment managed by flk";

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
          # Go toolchain
          go
          gopls
          gotools
          go-tools

          # Additional tools
          delve
          golangci-lint

          # User packages
        ];
        containerPackages = with pkgs; [
          bashInteractive
          coreutils
          findutils
          gnugrep
          git
        ];
        devEnv = {
          LANG = "en_US.UTF-8";
          LC_ALL = "en_US.UTF-8";
        };
        shellHook = ''
          echo "🐹 Go development environment ready!"
          echo "Go version: $(go version)"
          source .flk/hooks.sh

          # Custom commands will be added here
        '';
      in {
        devShells.default = pkgs.mkShell ({
            packages = devPackages;
            shellHook = shellHook;
          }
          // devEnv);
        packages.docker = pkgs.dockerTools.buildLayeredImage {
          name = "go-dev";
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
