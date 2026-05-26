{
  nixpkgs,
  flake-utils,
  profile-lib,
}: {
  src,
  systems ? null,
  nixpkgsOverride ? null,
}: _inputs: let
  np =
    if nixpkgsOverride != null
    then nixpkgsOverride
    else nixpkgs;
  lib = np.lib;

  configPath = src + "/config.nix";
  config =
    if builtins.pathExists configPath
    then import configPath
    else {};
  defaultProfile = config.defaultProfile or "";
  maxCombinations = config.maxCombinations or 3;

  eachSystem =
    if systems == null
    then flake-utils.lib.eachDefaultSystem
    else flake-utils.lib.eachSystem systems;

  pinsPath = src + "/pins.nix";
  pinsData =
    if builtins.pathExists pinsPath
    then import pinsPath
    else {
      sources = {};
      pinnedPackages = {};
    };
  pins = pinsData.sources or {};
  pinnedPackages = pinsData.pinnedPackages or {};

  fetchPin = ref: builtins.getFlake ref;

  profilesDir = src + "/profiles";
in
  eachSystem (
    system: let
      rustOverlayList =
        if pins ? rust-overlay
        then [(fetchPin pins.rust-overlay).overlays.default]
        else [];

      createPinnedOverlays = pp:
        builtins.map (
          pinName: let
            pkgDefs = pp.${pinName};
            pinnedPkgs = (fetchPin pins.${pinName}).legacyPackages.${system};
          in
            _final: _prev:
              builtins.listToAttrs (
                builtins.map (pkgDef: {
                  name = pkgDef.name;
                  value = pinnedPkgs.${pkgDef.pkg};
                })
                pkgDefs
              )
        ) (builtins.attrNames pp);

      overlays = rustOverlayList ++ (createPinnedOverlays pinnedPackages);

      pkgs = import np {
        inherit system overlays;
      };

      profileLib = profile-lib.lib {inherit pkgs;};

      profileFiles = builtins.readDir profilesDir;
      profileDefinitions = builtins.listToAttrs (
        map (file: {
          name = lib.removeSuffix ".nix" file;
          value = import (profilesDir + "/${file}") {inherit pkgs system;};
        })
        (builtins.filter
          (n: n != "default.nix" && lib.hasSuffix ".nix" n)
          (builtins.attrNames profileFiles))
      );
    in
      profileLib.mkProfileOutputs {
        inherit profileDefinitions maxCombinations;
      }
      // lib.optionalAttrs (defaultProfile != "") {defaultShell = defaultProfile;}
  )
