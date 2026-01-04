{
  pkgs,
  system,
}: {
  packages = [
    pkgs.git
    pkgs.curl
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
