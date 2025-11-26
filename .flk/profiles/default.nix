{
  pkgs,
  system,
}: let
  # Auto-import all .nix files in this directory
  entries = builtins.readDir ./.;
  nixFiles =
    builtins.filter
    (name: name != "default.nix" && pkgs.lib.hasSuffix ".nix" name)
    (builtins.attrNames entries);
in
  builtins.listToAttrs (map (file: {
      name = pkgs.lib.removeSuffix ".nix" file;
      value = import ./${file} {inherit pkgs system;};
    })
    nixFiles)
