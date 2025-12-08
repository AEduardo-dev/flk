system: let
  # Load all pin data
  pinsData = import ./pins.nix;
  pins = pinsData.sources;
  pinnedPackages = pinsData.pinnedPackages;

  fetchPin = ref: builtins.getFlake ref;

  rust-overlay = fetchPin pins.rust-overlay;

  # Dynamically create overlays for pinned packages
  createPinnedOverlays = pinnedPackages:
    builtins.map (
      pinName: let
        pkgDefs = pinnedPackages.${pinName};
        pinnedPkgs = (fetchPin pins.${pinName}).legacyPackages.${system};
      in
        final: prev:
          builtins.listToAttrs (
            builtins.map (pkgDef: {
              name = pkgDef.name;
              value = pinnedPkgs.${pkgDef.pkg};
            })
            pkgDefs
          )
    ) (builtins.attrNames pinnedPackages);
in
  [
    rust-overlay.overlays.default
  ]
  ++ (createPinnedOverlays pinnedPackages)
