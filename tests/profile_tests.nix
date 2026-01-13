{pkgs, ...}: {
  packages = [
    pkgs.git
    pkgs.curl
  ];

  envVars = {
    VAR1 = "value1";
    VAR2 = "value2";
    VAR3 = "value3";
  };

  commands = [
    {
      name = "test";
      script = ''
        echo "This is a test command"
      '';
    }
  ];

  shellHook = ''
    echo "Welcome to the development shell!"


  '';

  containerConfig = {
    Cmd = ["${pkgs.bashInteractive}/bin/bash"];
  };
}
