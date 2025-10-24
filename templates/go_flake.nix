{
  description = "Go development environment managed by flk";

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
            # Go toolchain
            go
            gopls
            gotools
            go-tools

            # Additional tools
            delve
            golangci-lint
          ];

          shellHook = ''
            echo "🐹 Go development environment ready!"
            echo "Go version: $(go version)"

            # Custom commands will be added here
          '';
        };
      }
    );
}
