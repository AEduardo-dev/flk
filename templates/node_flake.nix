{
  description = "Node.js development environment managed by flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Node.js and package managers
            nodejs_20
            nodePackages.npm
            nodePackages.pnpm
            yarn

            # Additional tools
            nodePackages.typescript
            nodePackages.eslint
            nodePackages.prettier
          ];

          shellHook = ''
            echo "📦 Node.js development environment ready!"
            echo "Node version: $(node --version)"
            echo "npm version: $(npm --version)"

            # Custom commands will be added here
          '';
        };
      }
    );
}
