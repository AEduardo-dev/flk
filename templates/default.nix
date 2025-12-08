inputs: let
  inherit (inputs) flake-utils nixpkgs profile-lib;
in
  flake-utils.lib.eachDefaultSystem (
    system: let
      overlays = import ./overlays.nix system;

      pkgs = import nixpkgs {
        inherit system overlays;
      };

      profileLib = profile-lib.lib {inherit pkgs;};

      profileFiles = builtins.readDir ./profiles;
      profileDefinitions = builtins.listToAttrs (
        map (file: {
          name = pkgs.lib.removeSuffix ".nix" file;
          value = import (./profiles + "/${file}") {inherit pkgs system;};
        })
        (builtins.filter
          (n: n != "default.nix" && pkgs.lib.hasSuffix ".nix" n)
          (builtins.attrNames profileFiles))
      );
    in
      profileLib.mkProfileOutputs {
        inherit profileDefinitions;
        maxCombinations = 3;
      }
  )
