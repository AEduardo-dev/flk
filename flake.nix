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
      # API documentation (cargo doc)
      flkApiDocs = pkgs.rustPlatform.buildRustPackage {
        pname = "${cargoToml.name}-api-docs";
        version = cargoToml.version;
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;

        buildPhase = ''
          cargo doc --no-deps --document-private-items
        '';

        installPhase = ''
          mkdir -p $out/docs
          cp -r target/doc/* $out/docs/
        '';

        doCheck = false;
      };

      # User guide documentation (mdBook)
      flkUserDocs = pkgs.stdenv.mkDerivation {
        pname = "${cargoToml.name}-user-docs";
        version = cargoToml.version;
        src = ./flk-book;

        nativeBuildInputs = [pkgs.mdbook];

        buildPhase = ''
          # Copy source to a writable directory
          cp -r $src $TMPDIR/docs
          chmod -R +w $TMPDIR/docs
          cd $TMPDIR/docs

          # Build the book
          mdbook build
        '';

        installPhase = ''
          mkdir -p $out/docs
          cp -r $TMPDIR/docs/book $out/docs/
        '';
      };

      # Combined documentation package
      flkDocs = pkgs.stdenv.mkDerivation {
        pname = "${cargoToml.name}-docs";
        version = cargoToml.version;

        dontUnpack = true;

        installPhase = ''
          mkdir -p $out/docs

          # Copy user guide to root
          cp -r ${flkUserDocs}/* $out/docs/

          # Copy API docs to /api subdirectory
          mkdir -p $out/api
          cp -r ${flkApiDocs}/* $out/docs/api/
        '';
      };
    in {
      packages = {
        flk = flkPackage;
        docs = flkDocs;
        api-docs = flkApiDocs;
        user-docs = flkUserDocs;
        default = flkPackage;
      };

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
