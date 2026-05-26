{
  description = "Development environment managed by flk";

  inputs = {
    flk.url = "github:AEduardo-dev/flk";
  };

  outputs = inputs: inputs.flk.lib.mkProject {src = ./.flk;} inputs;
}
