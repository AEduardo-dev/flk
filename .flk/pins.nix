{
  sources = {
    stable = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay = "github:oxalica/rust-overlay";
    pkgs-f720de5 = "github:NixOS/nixpkgs/f720de5";
    };

  pinnedPackages = {
    pkgs-f720de5 = [
            { pkg = "openssl"; name = "openssl@3.6.0"; }
    ];
    };
}
