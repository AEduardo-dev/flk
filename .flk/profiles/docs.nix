{pkgs, ...}: let
in {
  description = "Docs tooling for project";

  packages = [
    pkgs.mdbook
    pkgs.mdbook-plugins
    pkgs.mdbook-toc
    pkgs.mdbook-mermaid
  ];

  envVars = {
    LANG = "en_US.UTF-8";
    LC_ALL = "en_US.UTF-8";
  };

  commands = [];

  shellHook = ''
    echo "🛠️  Docs environment ready!"
    # Custom commands will be added here
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
