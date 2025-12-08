{
  sources = {
    stable = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay = "github:oxalica/rust-overlay";
    pkgs-abc1234 = "github:NixOS/nixpkgs/abc1234";
  };

  pinnedPackages = {
    pkgs-abc1234 = [
      {
        pkg = "git";
        name = "git@2.51.2";
      }
      {
        pkg = "curl";
        name = "curl@8.0.0";
      }
    ];
    pkgs-def5678 = [
      {
        pkg = "ripgrep";
        name = "ripgrep@14.1.0";
      }
    ];
  };
}
