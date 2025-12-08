system: let
  # Load pins
  pins = import ./pins.nix;

  fetchPin = ref: builtins. getFlake ref;

  rust-overlay = fetchPin pins. rust-overlay;

  # Define which packages to pin from which nixpkgs version
  pinnedPackages = {
    existing-overlay = [
      "git@latest"
    ];
  };

  # Dynamically create overlays for pinned packages
  createPinnedOverlays = pinnedPackages:
    builtins.map (
      pinName: let
        pkgNames = pinnedPackages.${pinName};
        pinnedPkgs = (fetchPin pins.${pinName}).legacyPackages.${system};
      in
        final: prev:
          builtins.listToAttrs (
            builtins.map (pkgName: {
              name = pkgName;
              value = pinnedPkgs.${pkgName};
            })
            pkgNames
          )
    ) (builtins.attrNames pinnedPackages);
in
  [
    rust-overlay.overlays.default
  ]
  ++ (createPinnedOverlays pinnedPackages)
