{
  description = "Node.js development environment managed by flk";

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
          nodejs = {
            packages = with pkgs; [
              nodejs_20
              nodePackages.npm
              nodePackages.pnpm
              yarn
              nodePackages.typescript
              nodePackages.eslint
              nodePackages.prettier
            ];

            envVars = {
              LANG = "en_US.UTF-8";
              LC_ALL = "en_US.UTF-8";
            };

            shellHook = ''
              echo "📦 Node.js development environment ready!"
              echo "Node version: $(node --version)"

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
          defaultShell = "nodejs";
          defaultImage = "nodejs";
        }
    );
}
