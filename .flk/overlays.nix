system: let
  # Load pins
  pins = import ./pins.nix;

  fetchPin = ref: builtins. getFlake ref;

  rust-overlay = fetchPin pins. rust-overlay;

  # Define which packages to pin from which nixpkgs version
  pinnedPackages = {
    # Key is the pin name from pins.nix, value is list of package names
    pkgs-for-nodejs = ["nodejs" "nodejs_20" "yarn" "npm"];
    pkgs-for-postgres = ["postgresql" "postgresql_15" "pgcli"];
    # Add more as needed:
    # pkgs-for-python = ["python311" "python311Packages.pip"];
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
