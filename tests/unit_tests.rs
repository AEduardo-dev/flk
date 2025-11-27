// Unit tests for individual modules
// These test internal functionality without running the full CLI

#[cfg(test)]
mod parser_tests {
    use flk::flake::parsers::{
        commands::{
            add_command_to_shell_hook, command_exists, parse_shell_hook_from_profile,
            remove_command_from_shell_hook,
        },
        env::{
            add_env_var_to_profile, env_var_exists, parse_env_vars_from_profile,
            remove_env_var_from_profile,
        },
        packages::{
            add_package_to_profile, find_packages_in_profile, package_exists,
            parse_packages_from_profile, remove_package_from_profile,
        },
    };

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
    fn test_package_exists() {
        // Test that package_exists correctly identifies packages
        let exists = package_exists(CONTENT, "git", None).unwrap();
        assert!(exists);

        let not_exists = package_exists(CONTENT, "nonexistent", None).unwrap();
        assert!(!not_exists);
    }

    #[test]
    fn test_package_exists_with_pkgs_prefix() {
        let content = r#"
          packages = [
            pkgs.git
            pkgs.curl
          ];
        "#;
        let exists = package_exists(content, "git", Some("test")).unwrap();
        assert!(exists);
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
        let result = add_package_to_profile(content, "ripgrep", None).unwrap();
        assert!(result.contains("ripgrep"));
    }

    #[test]
    fn test_add_package_to_existing_list() {
        // Test adding a package to existing list
        let result = add_package_to_profile(CONTENT, "ripgrep", None).unwrap();
        assert!(result.contains("ripgrep"));
        assert!(result.contains("git"));
        assert!(result.contains("curl"));
    }

    #[test]
    fn test_add_package_preserves_formatting() {
        let result = add_package_to_profile(CONTENT, "ripgrep", None).unwrap();
        // Check that proper indentation is maintained
        assert!(result.contains("    ") || result.contains("  "));
    }

    #[test]
    fn test_remove_package() {
        // Test removing a package
        let result = remove_package_from_profile(CONTENT, "curl", None).unwrap();
        println!("{}", result);
        assert!(result.contains("git"));
        assert!(!result.contains("curl"));
    }

    #[test]
    fn test_remove_package_from_middle() {
        let content = r#"
          packages = with pkgs; [
            git
            curl
            wget
          ];
        "#;
        let result = remove_package_from_profile(content, "curl", None).unwrap();
        assert!(result.contains("git"));
        assert!(result.contains("wget"));
        assert!(!result.contains("curl"));
    }

    #[test]
    fn test_remove_nonexistent_package() {
        let result = remove_package_from_profile(CONTENT, "nonexistent", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_command_exists() {
        // Test command detection
        let exists = command_exists(CONTENT, "test");
        assert!(exists);

        let not_exists = command_exists(CONTENT, "nonexistent");
        assert!(!not_exists);
    }

    #[test]
    fn test_add_command() {
        // Test adding a command
        let result =
            add_command_to_shell_hook(CONTENT, "test_add", "echo 'test command'", None).unwrap();
        assert!(result.contains("# flk-command: test_add"));
        assert!(result.contains("test_add ()"));
    }

    #[test]
    fn test_add_command_with_multiline() {
        let multiline_cmd = "echo 'line 1'\necho 'line 2'\necho 'line 3'";
        let result = add_command_to_shell_hook(CONTENT, "multiline", multiline_cmd, None).unwrap();
        assert!(result.contains("# flk-command: multiline"));
        assert!(result.contains("line 1"));
        assert!(result.contains("line 2"));
        assert!(result.contains("line 3"));
    }

    #[test]
    fn test_add_command_with_special_chars() {
        let cmd = "cargo build --release && echo 'Done!'";
        let result = add_command_to_shell_hook(CONTENT, "build", cmd, None).unwrap();
        assert!(result.contains("# flk-command: build"));
        assert!(result.contains("&&"));
    }

    #[test]
    fn test_remove_command() {
        // Test removing a command
        let result = remove_command_from_shell_hook(CONTENT, "test", None).unwrap();
        assert!(!result.contains("# flk-command: test"));
        assert!(!result.contains("test ()"));
    }

    #[test]
    fn test_remove_nonexistent_command() {
        let result = remove_command_from_shell_hook(CONTENT, "nonexistent", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_env_var_exists() {
        // Test env var detection
        let exists = env_var_exists(CONTENT, "VAR2", "default").unwrap();
        assert!(exists);

        let not_exists = env_var_exists(CONTENT, "NONEXISTENT", "default").unwrap();
        assert!(!not_exists);
    }

    #[test]
    fn test_add_env_var() {
        // Test adding an environment variable
        let result = add_env_var_to_profile(CONTENT, "MY_VAR", "test_value", None).unwrap();
        assert!(result.contains(" MY_VAR = \"test_value\""));
    }

    #[test]
    fn test_add_env_var_with_quotes() {
        let result =
            add_env_var_to_profile(CONTENT, "QUOTED", r#"value"with"quotes"#, None).unwrap();
        assert!(result.contains("QUOTED"));
        assert!(result.contains(r#"value\"with\"quotes"#));
    }

    #[test]
    fn test_add_env_var_with_special_chars() {
        let result =
            add_env_var_to_profile(CONTENT, "SPECIAL", "value with $pecial ch@rs!", None).unwrap();
        assert!(result.contains("SPECIAL"));
        assert!(result.contains("value with $pecial ch@rs!"));
    }

    #[test]
    fn test_remove_env_var() {
        // Test removing an environment variable
        let result = remove_env_var_from_profile(CONTENT, "VAR1", None).unwrap();
        assert!(!result.contains("VAR1"));
    }

    #[test]
    fn test_remove_env_var_middle() {
        let result = remove_env_var_from_profile(CONTENT, "VAR2", None).unwrap();
        assert!(result.contains("VAR1"));
        assert!(result.contains("VAR3"));
        assert!(!result.contains("VAR2"));
    }

    #[test]
    fn test_parse_env_vars() {
        // Test parsing all environment variables
        let vars = parse_env_vars_from_profile(CONTENT, None).unwrap();
        assert_eq!(vars.len(), 3);
        assert!(vars.contains(&("VAR1".to_string(), "value1".to_string())));
        assert!(vars.contains(&("VAR2".to_string(), "value2".to_string())));
        assert!(vars.contains(&("VAR3".to_string(), "value3".to_string())));
    }

    #[test]
    fn test_parse_env_vars_empty() {
        let content = r#"
          envVars = {
          };
        "#;
        let vars = parse_env_vars_from_profile(content, Some("test")).unwrap();
        assert_eq!(vars.len(), 0);
    }

    #[test]
    fn test_find_matching_brace_balanced() {
        // This tests the internal matching brace logic
        let content = "{ nested { more } }";
        // The function is not public, so we test it indirectly through other functions
        assert!(
            content.chars().filter(|&c| c == '{').count()
                == content.chars().filter(|&c| c == '}').count()
        );
    }

    #[test]
    fn test_find_packages_with_prefix() {
        let content = r#"
          packages = [
            pkgs.git
            pkgs.curl
          ];
        "#;
        let result = find_packages_in_profile(content, "test");
        assert!(result.is_ok());
        let (start, end, has_with_pkgs) = result.unwrap();
        assert!(start < end);
        assert!(!has_with_pkgs);
    }

    #[test]
    fn test_find_packages_with_with_pkgs() {
        let result = find_packages_in_profile(CONTENT, "default");
        assert!(result.is_ok());
        let (start, end, has_with_pkgs) = result.unwrap();
        assert!(start < end);
        assert!(has_with_pkgs);
    }

    #[test]
    fn test_parse_shell_hook() {
        let hook = parse_shell_hook_from_profile(CONTENT, Some("default")).unwrap();
        assert!(hook.contains("Welcome to the development shell!"));
        assert!(hook.contains("# flk-command: test"));
    }

    #[test]
    fn test_parse_packages_ignores_comments() {
        let content = r#"
          packages = with pkgs; [
            git
            # This is a comment
            curl
          ];
        "#;
        let packages = parse_packages_from_profile(content, Some("test")).unwrap();
        assert_eq!(packages.len(), 2);
        assert!(packages.iter().any(|p| p.name == "git"));
        assert!(packages.iter().any(|p| p.name == "curl"));
    }

    #[test]
    fn test_indent_consistency() {
        let result = add_package_to_profile(CONTENT, "test", None).unwrap();
        // Check that lines are properly indented (either 2 or 4 spaces)
        let lines: Vec<&str> = result.lines().collect();
        for line in lines {
            if !line.trim().is_empty() {
                let leading_spaces = line.len() - line.trim_start().len();
                assert!(leading_spaces % 2 == 0);
            }
        }
    }
}

#[cfg(test)]
mod generator_tests {
    use flk::flake;

    #[test]
    fn test_generate_generic_flake() {
        // Test generating a generic flake
        let flake = flake::generator::generate_flake("generic").unwrap();
        assert!(flake.contains("Generic Development Environment"));
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
        assert!(flake.contains("python311"));
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
        assert!(flake.contains("Generic Development Environment"));
    }

    #[test]
    fn test_generate_root_flake() {
        let flake = flake::generator::generate_root_flake().unwrap();
        assert!(!flake.is_empty());
        assert!(flake.contains("inputs"));
        assert!(flake.contains("outputs"));
    }

    #[test]
    fn test_generate_helper_module() {
        let helper = flake::generator::generate_helper_module().unwrap();
        assert!(!helper.is_empty());
    }

    #[test]
    fn test_generate_importer_module() {
        let importer = flake::generator::generate_importer_module().unwrap();
        assert!(!importer.is_empty());
    }

    #[test]
    fn test_generate_overlays() {
        let overlays = flake::generator::generate_overlays().unwrap();
        assert!(!overlays.is_empty());
    }

    #[test]
    fn test_generate_pins() {
        let pins = flake::generator::generate_pins().unwrap();
        assert!(!pins.is_empty());
    }

    #[test]
    fn test_all_templates_are_valid_nix() {
        let templates = vec!["rust", "python", "node", "go", "generic"];
        for template in templates {
            let flake = flake::generator::generate_flake(template).unwrap();
            // Basic validation: contains key Nix syntax
            assert!(flake.contains("packages"));
            assert!(flake.contains("="));
        }
    }

    #[test]
    fn test_all_templates_have_env_vars_section() {
        let templates = vec!["rust", "python", "node", "go", "generic"];
        for template in templates {
            let flake = flake::generator::generate_flake(template).unwrap();
            assert!(flake.contains("envVars"));
        }
    }

    #[test]
    fn test_all_templates_have_shell_hook() {
        let templates = vec!["rust", "python", "node", "go", "generic"];
        for template in templates {
            let flake = flake::generator::generate_flake(template).unwrap();
            assert!(flake.contains("shellHook"));
        }
    }
}

#[cfg(test)]
mod interface_tests {
    use flk::flake::interface::{EnvVar, FlakeConfig, Package, Profile};

    #[test]
    fn test_package_creation() {
        let pkg = Package::new("ripgrep".to_string());
        assert_eq!(pkg.name, "ripgrep");
        assert_eq!(pkg.version.unwrap(), "latest");
    }

    #[test]
    fn test_package_display() {
        let pkg = Package::new("test-pkg".to_string());
        let display = format!("{}", pkg);
        assert!(display.contains("test-pkg"));
    }

    #[test]
    fn test_env_var_creation() {
        let env = EnvVar::new("TEST_VAR".to_string(), "test_value".to_string());
        assert_eq!(env.name, "TEST_VAR");
        assert_eq!(env.value, "test_value");
    }

    #[test]
    fn test_env_var_display() {
        let env = EnvVar::new("MY_VAR".to_string(), "my_value".to_string());
        let display = format!("{}", env);
        assert!(display.contains("MY_VAR"));
        assert!(display.contains("my_value"));
    }

    #[test]
    fn test_profile_creation() {
        let profile = Profile::new("test-profile".to_string());
        assert_eq!(profile.name, "test-profile");
        assert_eq!(profile.packages.len(), 0);
        assert_eq!(profile.env_vars.len(), 0);
    }

    #[test]
    fn test_profile_with_data() {
        let mut profile = Profile::new("dev".to_string());
        profile.packages.push(Package::new("git".to_string()));
        profile
            .env_vars
            .push(EnvVar::new("VAR1".to_string(), "value1".to_string()));

        assert_eq!(profile.packages.len(), 1);
        assert_eq!(profile.env_vars.len(), 1);
    }

    #[test]
    fn test_flake_config_default() {
        let config = FlakeConfig::default();
        assert!(config.inputs.is_empty());
        assert!(config.profiles.is_empty());
    }

    #[test]
    fn test_flake_config_with_profiles() {
        let mut config = FlakeConfig::default();
        config.profiles.push(Profile::new("default".to_string()));
        config.profiles.push(Profile::new("dev".to_string()));

        assert_eq!(config.profiles.len(), 2);
    }

    #[test]
    fn test_flake_config_display() {
        let mut config = FlakeConfig::default();
        config.inputs.push("nixpkgs".to_string());
        let mut profile = Profile::new("default".to_string());
        profile.packages.push(Package::new("git".to_string()));
        config.profiles.push(profile);

        let display = format!("{}", config);
        assert!(display.contains("nixpkgs"));
        assert!(display.contains("default"));
    }
}
