inputs: let
  inherit (inputs) flake-utils nixpkgs profile-lib;
in
  flake-utils.lib.eachDefaultSystem (
    system: let
      pkgs = import nixpkgs {inherit system;};
      profileLib = profile-lib.lib {inherit pkgs;};

      # Auto-import all profiles from .flk/profiles/
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
        maxCombinations = 3; # rust, node, rust-node, etc.
      }
  )
