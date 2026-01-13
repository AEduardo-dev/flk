{pkgs, ...}: let
in {
  description = "Generic Development Environment";

  packages = [
    # User packages
  ];

  envVars = {
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

  commands = [];

  shellHook = ''
    echo "🛠️  Development environment ready!"
    # Custom commands will be added here
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
