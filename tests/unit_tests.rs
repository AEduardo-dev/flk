// Unit tests for individual modules
// These test internal functionality without running the full CLI

#[cfg(test)]
mod parser_tests {
    use std::fs;
    use tempfile::NamedTempFile;

    // Helper to create a test flake file
    fn create_test_flake(content: &str) -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), content).unwrap();
        file
    }

    #[test]
    fn test_parse_description() {
        let content = r#"
{
  description = "Test flake description";
  inputs = {};
}
"#;
        let file = create_test_flake(content);

        // You would need to make parse_flake public or create a test module
        // For now, this is a template showing how to test
        let path = file.path().to_str().unwrap();

        // Example assertion (you'll need to import your parser module)
        // let config = flake::parser::parse_flake(path).unwrap();
        // assert_eq!(config.description, "Test flake description");
    }

    #[test]
    fn test_package_exists() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    packages = with pkgs; [
      git
      ripgrep
      curl
    ];
  };
}
"#;
        // Test that package_exists correctly identifies packages
        // let exists = flake::parser::package_exists(content, "ripgrep").unwrap();
        // assert!(exists);

        // let not_exists = flake::parser::package_exists(content, "nonexistent").unwrap();
        // assert!(!not_exists);
    }

    #[test]
    fn test_add_package_to_empty_list() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    packages = with pkgs; [
    ];
  };
}
"#;
        // Test adding a package to empty list
        // let result = flake::parser::add_package_inputs(content, "ripgrep").unwrap();
        // assert!(result.contains("ripgrep"));
    }

    #[test]
    fn test_add_package_to_existing_list() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    packages = with pkgs; [
      git
      curl
    ];
  };
}
"#;
        // Test adding a package to existing list
        // let result = flake::parser::add_package_inputs(content, "ripgrep").unwrap();
        // assert!(result.contains("ripgrep"));
        // assert!(result.contains("git"));
        // assert!(result.contains("curl"));
    }

    #[test]
    fn test_remove_package() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    packages = with pkgs; [
      git
      ripgrep
      curl
    ];
  };
}
"#;
        // Test removing a package
        // let result = flake::parser::remove_package_inputs(content, "ripgrep").unwrap();
        // assert!(!result.contains("ripgrep"));
        // assert!(result.contains("git"));
        // assert!(result.contains("curl"));
    }

    #[test]
    fn test_command_exists() {
        let content = r#"
shellHook = ''
  echo "Welcome"
  
  # flk-command: test
  test () {
    echo "Test command"
  }
'';
"#;
        // Test command detection
        // let exists = flake::parser::command_exists(content, "test");
        // assert!(exists);

        // let not_exists = flake::parser::command_exists(content, "nonexistent");
        // assert!(!not_exists);
    }

    #[test]
    fn test_add_command() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    shellHook = ''
      echo "Welcome"
    '';
  };
}
"#;
        // Test adding a command
        // let result = flake::parser::add_command_to_shell_hook(
        //     content,
        //     "test",
        //     "echo 'test command'"
        // ).unwrap();
        // assert!(result.contains("# flk-command: test"));
        // assert!(result.contains("test ()"));
    }

    #[test]
    fn test_remove_command() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    shellHook = ''
      echo "Welcome"
      
      # flk-command: test
      test () {
        echo "Test command"
      }
    '';
  };
}
"#;
        // Test removing a command
        // let result = flake::parser::remove_command_from_shell_hook(content, "test").unwrap();
        // assert!(!result.contains("# flk-command: test"));
        // assert!(!result.contains("test ()"));
    }

    #[test]
    fn test_env_var_exists() {
        let content = r#"
shellHook = ''
  export MY_VAR="test"
  export ANOTHER_VAR="value"
'';
"#;
        // Test env var detection
        // let exists = flake::parser::env_var_exists(content, "MY_VAR").unwrap();
        // assert!(exists);

        // let not_exists = flake::parser::env_var_exists(content, "NONEXISTENT").unwrap();
        // assert!(!not_exists);
    }

    #[test]
    fn test_add_env_var() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    shellHook = ''
      echo "Welcome"
    '';
  };
}
"#;
        // Test adding an environment variable
        // let result = flake::parser::add_env_var(content, "MY_VAR", "test_value").unwrap();
        // assert!(result.contains("# flk-env: MY_VAR"));
        // assert!(result.contains("export MY_VAR=\"test_value\""));
    }

    #[test]
    fn test_remove_env_var() {
        let content = r#"
{
  devShells.default = pkgs.mkShell {
    shellHook = ''
      # flk-env: MY_VAR
      export MY_VAR="test"
    '';
  };
}
"#;
        // Test removing an environment variable
        // let result = flake::parser::remove_env_var(content, "MY_VAR").unwrap();
        // assert!(!result.contains("# flk-env: MY_VAR"));
        // assert!(!result.contains("export MY_VAR"));
    }

    #[test]
    fn test_parse_env_vars() {
        let content = r#"
shellHook = ''
  export VAR1="value1"
  export VAR2="value2"
  echo "test"
  export VAR3="value3"
'';
"#;
        // Test parsing all environment variables
        // let vars = flake::parser::parse_env_vars(content).unwrap();
        // assert_eq!(vars.len(), 3);
        // assert!(vars.contains(&("VAR1".to_string(), "value1".to_string())));
        // assert!(vars.contains(&("VAR2".to_string(), "value2".to_string())));
        // assert!(vars.contains(&("VAR3".to_string(), "value3".to_string())));
    }
}

#[cfg(test)]
mod generator_tests {
    #[test]
    fn test_generate_generic_flake() {
        // Test generating a generic flake
        // let flake = flake::generator::generate_flake("generic").unwrap();
        // assert!(flake.contains("Development environment managed by flk"));
        // assert!(flake.contains("nixpkgs"));
    }

    #[test]
    fn test_generate_rust_flake() {
        // Test generating a Rust flake
        // let flake = flake::generator::generate_flake("rust").unwrap();
        // assert!(flake.contains("Rust development environment"));
        // assert!(flake.contains("rust-bin.stable.latest.default"));
    }

    #[test]
    fn test_generate_python_flake() {
        // Test generating a Python flake
        // let flake = flake::generator::generate_flake("python").unwrap();
        // assert!(flake.contains("Python development environment"));
        // assert!(flake.contains("python311"));
    }

    #[test]
    fn test_generate_node_flake() {
        // Test generating a Node.js flake
        // let flake = flake::generator::generate_flake("node").unwrap();
        // assert!(flake.contains("Node.js development environment"));
        // assert!(flake.contains("nodejs"));
    }

    #[test]
    fn test_generate_go_flake() {
        // Test generating a Go flake
        // let flake = flake::generator::generate_flake("go").unwrap();
        // assert!(flake.contains("Go development environment"));
        // assert!(flake.contains("go"));
    }

    #[test]
    fn test_unknown_template_defaults_to_generic() {
        // Test that unknown templates fall back to generic
        // let flake = flake::generator::generate_flake("unknown").unwrap();
        // assert!(flake.contains("Development environment managed by flk"));
    }
}

#[cfg(test)]
mod interface_tests {
    // use flake::interface::{Package, EnvVar, FlakeConfig};

    #[test]
    fn test_package_creation() {
        // let pkg = Package::new("ripgrep".to_string());
        // assert_eq!(pkg.name, "ripgrep");
        // assert_eq!(pkg.version, None);
    }

    #[test]
    fn test_package_with_version() {
        // let pkg = Package::with_version("ripgrep".to_string(), "13.0.0".to_string());
        // assert_eq!(pkg.name, "ripgrep");
        // assert_eq!(pkg.version, Some("13.0.0".to_string()));
    }

    #[test]
    fn test_env_var_creation() {
        // let env = EnvVar::new("TEST_VAR".to_string(), "test_value".to_string());
        // assert_eq!(env.name, "TEST_VAR");
        // assert_eq!(env.value, "test_value");
    }

    #[test]
    fn test_flake_config_default() {
        // let config = FlakeConfig::default();
        // assert!(config.description.is_empty());
        // assert!(config.inputs.is_empty());
        // assert!(config.packages.is_empty());
        // assert!(config.env_vars.is_empty());
        // assert!(config.shell_hook.is_empty());
    }
}
