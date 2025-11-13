// Unit tests for individual modules
// These test internal functionality without running the full CLI

#[cfg(test)]
mod parser_tests {
    use flk::flake;

    const CONTENT: &str = r#"
    {
      description = "Test flake description";

      inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        profile-lib.url = "github:AEduardo-dev/nix-profile-lib";
      };

      outputs = {
        self,
        flake-utils,
        nixpkgs,
        profile-lib,
      }:
        flake-utils.lib.eachDefaultSystem (system: let
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          profileLib = profile-lib.lib {inherit pkgs;};

          profileDefinitions = {
            default = {
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
                  echo "This is a test command"
                }
              '';

              containerConfig = {
                Cmd = ["${pkgs.bashInteractive}/bin/bash"];
              };
            };
          };
        in
          profileLib.mkProfileOutputs {
            inherit profileDefinitions;
            defaultShell = "default";
            defaultImage = "default";
          });
    }
    "#;

    #[test]
    fn test_parse_description() {
        let description = flake::parser::parse_description(&CONTENT);
        assert_eq!(description, "Test flake description");
    }

    #[test]
    fn test_package_exists() {
        // Test that package_exists correctly identifies packages
        let exists = flake::parser::package_exists(CONTENT, "git", None).unwrap();
        assert!(exists);

        let not_exists = flake::parser::package_exists(CONTENT, "nonexistent", None).unwrap();
        assert!(!not_exists);
    }

    #[test]
    fn test_add_package_to_empty_list() {
        let content = r#"
    ...
      test_package_existskgs = import nixpkgs {
        inherit system overlays;
      };

      profileLib = profile-lib.lib {inherit pkgs;};

      profileDefinitions = {
        default = {
          packages = with pkgs; [
          ];
        };
      };
    in
      profileLib.mkProfileOutputs {
...
"#;
        // Test adding a package to empty list
        let result = flake::parser::add_package_to_profile(content, "ripgrep", None).unwrap();
        assert!(result.contains("ripgrep"));
    }

    #[test]
    fn test_add_package_to_existing_list() {
        // Test adding a package to existing list
        let result = flake::parser::add_package_to_profile(CONTENT, "ripgrep", None).unwrap();
        assert!(result.contains("ripgrep"));
        assert!(result.contains("git"));
        assert!(result.contains("curl"));
    }

    #[test]
    fn test_remove_package() {
        // Test removing a package
        let result = flake::parser::remove_package_from_profile(CONTENT, "curl", None).unwrap();
        println!("{}", result);
        assert!(result.contains("git"));
        assert!(!result.contains("curl"));
    }

    #[test]
    fn test_command_exists() {
        // Test command detection
        let exists = flake::parser::command_exists(CONTENT, "test");
        assert!(exists);

        let not_exists = flake::parser::command_exists(CONTENT, "nonexistent");
        assert!(!not_exists);
    }

    #[test]
    fn test_add_command() {
        // Test adding a command
        let result = flake::parser::add_command_to_shell_hook(
            CONTENT,
            "test_add",
            "echo 'test command'",
            None,
        )
        .unwrap();
        assert!(result.contains("# flk-command: test_add"));
        assert!(result.contains("test_add ()"));
    }

    #[test]
    fn test_remove_command() {
        // Test removing a command
        let result = flake::parser::remove_command_from_shell_hook(CONTENT, "test", None).unwrap();
        assert!(!result.contains("# flk-command: test"));
        assert!(!result.contains("test ()"));
    }

    #[test]
    fn test_env_var_exists() {
        // Test env var detection
        let exists = flake::parser::env_var_exists(CONTENT, "VAR2", "default").unwrap();
        assert!(exists);

        let not_exists = flake::parser::env_var_exists(CONTENT, "NONEXISTENT", "default").unwrap();
        assert!(!not_exists);
    }

    #[test]
    fn test_add_env_var() {
        // Test adding an environment variable
        let result =
            flake::parser::add_env_var_to_profile(CONTENT, "MY_VAR", "test_value", None).unwrap();
        assert!(result.contains(" MY_VAR = \"test_value\""));
    }

    #[test]
    fn test_remove_env_var() {
        // Test removing an environment variable
        let result = flake::parser::remove_env_var_from_profile(CONTENT, "VAR1", None).unwrap();
        assert!(!result.contains("VAR1"));
    }

    #[test]
    fn test_parse_env_vars() {
        // Test parsing all environment variables
        let vars = flake::parser::parse_env_vars_from_profile(CONTENT, None).unwrap();
        assert_eq!(vars.len(), 3);
        assert!(vars.contains(&("VAR1".to_string(), "value1".to_string())));
        assert!(vars.contains(&("VAR2".to_string(), "value2".to_string())));
        assert!(vars.contains(&("VAR3".to_string(), "value3".to_string())));
    }
}

#[cfg(test)]
mod generator_tests {
    use flk::flake;

    #[test]
    fn test_generate_generic_flake() {
        // Test generating a generic flake
        let flake = flake::generator::generate_flake("generic").unwrap();
        assert!(flake.contains("Development environment managed by flk"));
        assert!(flake.contains("nixpkgs"));
    }

    #[test]
    fn test_generate_rust_flake() {
        // Test generating a Rust flake
        let flake = flake::generator::generate_flake("rust").unwrap();
        assert!(flake.contains("Rust development environment"));
        assert!(flake.contains("rust-bin.stable.latest.default"));
    }

    #[test]
    fn test_generate_python_flake() {
        // Test generating a Python flake
        let flake = flake::generator::generate_flake("python").unwrap();
        assert!(flake.contains("Python development environment"));
        assert!(flake.contains("python312"));
    }

    #[test]
    fn test_generate_node_flake() {
        // Test generating a Node.js flake
        let flake = flake::generator::generate_flake("node").unwrap();
        assert!(flake.contains("Node.js development environment"));
        assert!(flake.contains("nodejs"));
    }

    #[test]
    fn test_generate_go_flake() {
        // Test generating a Go flake
        let flake = flake::generator::generate_flake("go").unwrap();
        assert!(flake.contains("Go development environment"));
        assert!(flake.contains("go"));
    }

    #[test]
    fn test_unknown_template_defaults_to_generic() {
        // Test that unknown templates fall back to generic
        let flake = flake::generator::generate_flake("unknown").unwrap();
        assert!(flake.contains("Development environment managed by flk"));
    }
}

#[cfg(test)]
mod interface_tests {
    use flk::flake::interface::{EnvVar, FlakeConfig, Package};

    #[test]
    fn test_package_creation() {
        let pkg = Package::new("ripgrep".to_string());
        assert_eq!(pkg.name, "ripgrep");
        assert_eq!(pkg.version.unwrap(), "latest");
    }

    #[test]
    fn test_env_var_creation() {
        let env = EnvVar::new("TEST_VAR".to_string(), "test_value".to_string());
        assert_eq!(env.name, "TEST_VAR");
        assert_eq!(env.value, "test_value");
    }

    #[test]
    fn test_flake_config_default() {
        let config = FlakeConfig::default();
        assert!(config.description.is_empty());
        assert!(config.inputs.is_empty());
        assert!(config.profiles.is_empty());
    }
}
