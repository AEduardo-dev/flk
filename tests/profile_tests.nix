{
  pkgs,
  system,
}: {
  packages = with pkgs; [
    git
    curl
  ];

  envVars = {
    VAR1 = "value1";
    VAR2 = "value2";
    VAR3 = "value3";
  };

  shellHook = ''
    echo "Welcome to the development shell!"

    # flk-command: test
    test () {
      echo test
    }
  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
