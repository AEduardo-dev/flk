system: let
  # Load pins
  pins = import ./pins.nix;

  fetchPin = ref: builtins.getFlake ref;

  rust-overlay = fetchPin pins. rust-overlay;
in [
  rust-overlay. overlays. default
]
