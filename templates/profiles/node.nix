{pkgs, ...}: let
in {
  description = "Node.js development environment";

  packages = [
    pkgs.nodejs_20
    pkgs.nodePackages.npm
    pkgs.nodePackages.pnpm
    pkgs.yarn
    pkgs.nodePackages.typescript
    pkgs.nodePackages.javascript
    pkgs.nodePackages.eslint
    pkgs.nodePackages.prettier
  ];

  envVars = {
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

  commands = [];

  shellHook = ''
    echo "📦 Node.js development environment ready!"
    echo "Node version: $(node --version)"

    # Custom commands will be added here
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
