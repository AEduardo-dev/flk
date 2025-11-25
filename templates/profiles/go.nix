{pkgs}: let
in {
  description = "Go development environment";

  packages = with pkgs; [
    go
    gopls
    gotools
    go-tools
    delve
    golangci-lint
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
