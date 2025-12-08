{pkgs, ...}: let
in {
  description = "Go development environment";

  packages = [
    pkgs.go
    pkgs.gopls
    pkgs.gotools
    pkgs.go-tools
    pkgs.delve
    pkgs.golangci-lint
  ];

  envVars = {
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

  shellHook = ''
    echo "🐹 Go development environment ready!"
    echo "Go version: $(go version)"

    # Custom commands will be added here
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
