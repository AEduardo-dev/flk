{
  stable = "github:NixOS/nixpkgs/nixos-25.05";
  rust-overlay = "github:oxalica/rust-overlay";

  # Pinned package sources
  pkgs-for-nodejs = "github:NixOS/nixpkgs/a1b2c3d4e5f6";
  pkgs-for-postgres = "github:NixOS/nixpkgs/f6e5d4c3b2a1";
  pkgs-for-python = "github:NixOS/nixpkgs/1234567890ab";
}
