{
  description = "Go development environment managed by flk";

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
          go = {
            packages = with pkgs; [
              go
              gopls
              gotools
              go-tools
              delve
              golangci-lint
            ];

            envVars = {
              LANG = "en_US.UTF-8";
              LC_ALL = "en_US.UTF-8";
            };

            shellHook = ''
              echo "🐹 Go development environment ready!"
              echo "Go version: $(go version)"

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
          defaultShell = "go";
          defaultImage = "go";
        }
    );
}
