{
  description = "Node.js development environment managed by flk";

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
          # Node.js and package managers
          nodejs_20
          nodePackages.npm
          nodePackages.pnpm
          yarn

          # Additional tools
          nodePackages.typescript
          nodePackages.eslint
          nodePackages.prettier

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
          echo "📦 Node.js development environment ready!"
          echo "Node version: $(node --version)"
          echo "npm version: $(npm --version)"
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
          name = "nodejs-dev";
          tag = "latest";
          contents = devPackages ++ containerPackages;
          config = {
            Cmd = ["${pkgs.bashInteractive}/bin/bash"];
            Env = pkgs.lib.mapAttrsToList (name: value: "${name}=${value}") devEnv;
            WorkingDir = "/workspace";
          };
        };
        packages.podman = pkgs.dockerTools.buildLayeredImage {
          name = "nodejs-dev";
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
