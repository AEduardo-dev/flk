{
  pkgs,
  system,
}: let
in {
  description = "Node.js development environment";

  packages = with pkgs; [
    nodejs_20
    nodePackages.npm
    nodePackages.pnpm
    yarn
    nodePackages.typescript
    nodePackages.javascript
    nodePackages.eslint
    nodePackages.prettier
  ];

  envVars = {
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

  shellHook = ''
    echo "📦 Node.js development environment ready!"
    echo "Node version: $(node --version)"

    # Custom commands will be added here
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
