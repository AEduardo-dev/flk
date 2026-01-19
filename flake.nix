{
  description = "Development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    profile-lib.url = "github:AEduardo-dev/nix-profile-lib";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: let
    base = import ./.flk inputs;
    cargoToml = (nixpkgs.lib.importTOML ./Cargo.toml).package;
  in
    nixpkgs.lib.recursiveUpdate base
    (flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};

      flkPackage = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.name;
        version = cargoToml.version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
        doCheck = false; # Disable tests: nixpkgs exposure is limited in sandboxed builds
      };
    in {
      packages.flk = flkPackage;
      apps.flk = {
        type = "app";
        program = "${flkPackage}/bin/${cargoToml.name}";
      };

      # Overlay to expose flk as pkgs.flk
      overlay = final: prev: {
        flk = self.packages.${final.system}.flk;
      };
    }));
}
