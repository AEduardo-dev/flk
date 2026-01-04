// Unit tests for individual modules
// These test internal functionality without running the full CLI

#[cfg(test)]
mod parser_tests {
    use flk::flake::parsers::{
        commands::parse_shell_hook_section,
        env::parse_env_vars_section,
        overlays::{parse_overlay_section, parse_sources_section},
        packages::parse_packages_section,
    };

    const PROFILE_CONTENT: &str = include_str!("profile_tests.nix");
    const PINS_CONTENT: &str = include_str!("pins_tests.nix");

    #[test]
    fn test_package_exists() {
        // Test that package_exists correctly identifies packages
        let section = parse_packages_section(PROFILE_CONTENT).unwrap();
        let exists = section.package_exists("git");
        assert!(exists);

        let not_exists = section.package_exists("nonexistent");
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
        let section = parse_packages_section(content).unwrap();
        let exists = section.package_exists("git");
        assert!(exists);
    }

    #[test]
    fn test_add_package_to_empty_list() {
        let content = r#"
pkgs = import nixpkgs {
    inherit system overlays;
  };

  profileLib = profile-lib. lib {inherit pkgs;};

  profileDefinitions = {
    default = {
      packages = with pkgs; [
      ];
    };
  };
"#;
        let section = parse_packages_section(content).unwrap();
        // Test adding a package to empty list
        let result = section.add_package(content, "ripgrep", None);
        assert!(result.contains("ripgrep"));
    }

    #[test]
    fn test_add_package_to_existing_list() {
        let section = parse_packages_section(PROFILE_CONTENT).unwrap();
        // Test adding a package to existing list
        let result = section.add_package(PROFILE_CONTENT, "ripgrep", None);
        assert!(result.contains("ripgrep"));
        assert!(result.contains("git"));
        assert!(result.contains("curl"));
    }

    #[test]
    fn test_add_package_preserves_formatting() {
        let section = parse_packages_section(PROFILE_CONTENT).unwrap();
        let result = section.add_package(PROFILE_CONTENT, "ripgrep", None);
        // Check that proper indentation is maintained
        assert!(result.contains("    ") || result.contains("  "));
    }

    #[test]
    fn test_remove_package() {
        let section = parse_packages_section(PROFILE_CONTENT).unwrap();
        // Test removing a package
        let result = section.remove_package(PROFILE_CONTENT, "curl").unwrap();
        println!("{}", result);
        assert!(result.contains("git"));
        assert!(!result.contains("curl"));
    }

    #[test]
    fn test_remove_package_from_middle() {
        let content = r#"
          packages = [
            pkgs.git
            pkgs.curl
            pkgs.wget
          ];
        "#;
        let section = parse_packages_section(content).unwrap();
        let result = section.remove_package(content, "curl").unwrap();
        assert!(result.contains("git"));
        assert!(result.contains("wget"));
        assert!(!result.contains("curl"));
    }

    #[test]
    fn test_remove_nonexistent_package() {
        let section = parse_packages_section(PROFILE_CONTENT).unwrap();
        let result = section.remove_package(PROFILE_CONTENT, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_command_exists() {
        let section = parse_shell_hook_section(PROFILE_CONTENT).unwrap();
        // Test command detection
        let exists = section.command_exists(PROFILE_CONTENT, "test");
        assert!(exists);

        let not_exists = section.command_exists(PROFILE_CONTENT, "nonexistent");
        assert!(!not_exists);
    }

    #[test]
    fn test_add_command() {
        let section = parse_shell_hook_section(PROFILE_CONTENT).unwrap();
        // Test adding a command
        let result = section.add_command(PROFILE_CONTENT, "test_add", "echo 'test command'");
        assert!(result.contains("# flk-command: test_add"));
        assert!(result.contains("test_add ()"));
    }

    #[test]
    fn test_add_command_with_multiline() {
        let section = parse_shell_hook_section(PROFILE_CONTENT).unwrap();
        let multiline_cmd = "echo 'line 1'\necho 'line 2'\necho 'line 3'";
        let result = section.add_command(PROFILE_CONTENT, "multiline", multiline_cmd);
        assert!(result.contains("# flk-command: multiline"));
        assert!(result.contains("line 1"));
        assert!(result.contains("line 2"));
        assert!(result.contains("line 3"));
    }

    #[test]
    fn test_add_command_with_special_chars() {
        let cmd = "cargo build --release && echo 'Done!'";
        let section = parse_shell_hook_section(PROFILE_CONTENT).unwrap();
        let result = section.add_command(PROFILE_CONTENT, "build", cmd);
        assert!(result.contains("# flk-command: build"));
        assert!(result.contains("&&"));
    }

    #[test]
    fn test_remove_command() {
        let section = parse_shell_hook_section(PROFILE_CONTENT).unwrap();
        // Test removing a command
        let result = section.remove_command(PROFILE_CONTENT, "test").unwrap();
        assert!(!result.contains("# flk-command: test"));
        assert!(!result.contains("test ()"));
    }

    #[test]
    fn test_remove_nonexistent_command() {
        let section = parse_shell_hook_section(PROFILE_CONTENT).unwrap();
        let result = section.remove_command(PROFILE_CONTENT, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_env_var_exists() {
        let section = parse_env_vars_section(PROFILE_CONTENT).unwrap();
        // Test env var detection
        let exists = section.env_var_exists("VAR2").unwrap();
        assert!(exists);

        let not_exists = section.env_var_exists("NONEXISTENT").unwrap();
        assert!(!not_exists);
    }

    #[test]
    fn test_add_env_var() {
        let section = parse_env_vars_section(PROFILE_CONTENT).unwrap();
        // Test adding an environment variable
        let result = section.add_env_var(PROFILE_CONTENT, "MY_VAR", "test_value");
        assert!(result.contains(" MY_VAR = \"test_value\""));
    }

    #[test]
    fn test_add_env_var_with_quotes() {
        let section = parse_env_vars_section(PROFILE_CONTENT).unwrap();
        let result = section.add_env_var(PROFILE_CONTENT, "QUOTED", r#"value"with"quotes"#);
        assert!(result.contains("QUOTED"));
        assert!(result.contains(r#"value"with"quotes"#));
    }

    #[test]
    fn test_add_env_var_with_special_chars() {
        let section = parse_env_vars_section(PROFILE_CONTENT).unwrap();
        let result = section.add_env_var(PROFILE_CONTENT, "SPECIAL", "value with $pecial ch@rs!");
        assert!(result.contains("SPECIAL"));
        assert!(result.contains("value with $pecial ch@rs!"));
    }

    #[test]
    fn test_remove_env_var() {
        let section = parse_env_vars_section(PROFILE_CONTENT).unwrap();
        // Test removing an environment variable
        let result = section.remove_env_var(PROFILE_CONTENT, "VAR1").unwrap();
        assert!(!result.contains("VAR1"));
    }

    #[test]
    fn test_remove_env_var_middle() {
        let section = parse_env_vars_section(PROFILE_CONTENT).unwrap();
        let result = section.remove_env_var(PROFILE_CONTENT, "VAR2").unwrap();
        assert!(result.contains("VAR1"));
        assert!(result.contains("VAR3"));
        assert!(!result.contains("VAR2"));
    }

    #[test]
    fn test_parse_env_vars() {
        let section = parse_env_vars_section(PROFILE_CONTENT).unwrap();
        // Test parsing all environment variables
        assert_eq!(section.entries.len(), 3);
        let vars: Vec<(String, String)> = section
            .entries
            .iter()
            .map(|e| (e.name.clone(), e.value.clone()))
            .collect();
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
        let section = parse_env_vars_section(content).unwrap();
        assert_eq!(section.entries.len(), 0);
    }

    #[test]
    fn test_parse_shell_hook() {
        let section = parse_shell_hook_section(PROFILE_CONTENT).unwrap();
        let hook = &PROFILE_CONTENT[section.content_start..section.content_end];
        assert!(hook.contains("Welcome to the development shell!"));
        assert!(hook.contains("# flk-command: test"));
    }

    #[test]
    fn test_parse_packages_ignores_comments() {
        let content = r#"
          packages = [
            pkgs.git
            # This is a comment
            pkgs.curl
          ];
        "#;
        let section = parse_packages_section(content).unwrap();
        let packages: Vec<_> = section.entries.iter().collect();
        assert_eq!(packages.len(), 2);
        assert!(packages.iter().any(|p| p.name == "git"));
        assert!(packages.iter().any(|p| p.name == "curl"));
    }

    // ========================================================================
    // SOURCES TESTS
    // ========================================================================

    #[test]
    fn test_source_exists_with_existing_source() {
        let section = parse_sources_section(PINS_CONTENT).unwrap();
        let result = section.source_exists("pkgs-abc1234");
        assert!(result);
    }

    #[test]
    fn test_source_exists_with_nonexistent_source() {
        let section = parse_sources_section(PINS_CONTENT).unwrap();
        let result = section.source_exists("pkgs-nonexistent");
        assert!(!result);
    }

    #[test]
    fn test_source_exists_with_empty_content() {
        let content = r#"{
      sources = {
      };
      pinnedPackages = {
      };
    }"#;
        let section = parse_sources_section(content).unwrap();
        let result = section.source_exists("my-source");
        assert!(!result);
    }

    #[test]
    fn test_add_source() {
        let mut section = parse_sources_section(PINS_CONTENT).unwrap();
        section
            .add_source("pkgs-new123", "github:NixOS/nixpkgs/new123")
            .unwrap();

        assert!(section.entries.iter().any(|s| s.name == "pkgs-new123"));
        assert!(section
            .entries
            .iter()
            .any(|s| s.reference == "github:NixOS/nixpkgs/new123"));
    }

    #[test]
    fn test_remove_source() {
        let mut section = parse_sources_section(PINS_CONTENT).unwrap();
        section.remove_source("pkgs-abc1234").unwrap();
        assert!(!section.source_exists("pkgs-abc1234"));
        assert!(section.source_exists("pkgs-def5678")); // Other sources still there
    }

    #[test]
    fn test_remove_nonexistent_source() {
        let mut section = parse_sources_section(PINS_CONTENT).unwrap();
        let result = section.remove_source("pkgs-nonexistent");
        assert!(result.is_err());
    }

    // ========================================================================
    // PIN ENTRY TESTS
    // ========================================================================

    #[test]
    fn test_pin_entry_exists_with_existing_entry() {
        let section = parse_overlay_section(PINS_CONTENT).unwrap();
        assert!(section.pin_entry_exists("pkgs-abc1234"));
    }

    #[test]
    fn test_pin_entry_exists_with_nonexistent_entry() {
        let section = parse_overlay_section(PINS_CONTENT).unwrap();
        assert!(!section.pin_entry_exists("pkgs-nonexistent"));
    }

    #[test]
    fn test_pin_entry_exists_with_empty_content() {
        let content = r#"{
      sources = {
      };
      pinnedPackages = {
      };
    }"#;
        let section = parse_overlay_section(content).unwrap();
        assert!(!section.pin_entry_exists("my-pin"));
    }

    #[test]
    fn test_add_pin_entry() {
        let mut section = parse_overlay_section(PINS_CONTENT).unwrap();
        section.add_pin_entry("pkgs-new456").unwrap();

        assert!(section.pin_entry_exists("pkgs-new456"));
    }

    #[test]
    fn test_remove_pin_entry() {
        let mut section = parse_overlay_section(PINS_CONTENT).unwrap();
        section.remove_pin_entry("pkgs-abc1234").unwrap();
        assert!(!section.pin_entry_exists("pkgs-abc1234"));
        assert!(section.pin_entry_exists("pkgs-def5678")); // Other entries still there
    }

    #[test]
    fn test_remove_nonexistent_pin_entry() {
        let mut section = parse_overlay_section(PINS_CONTENT).unwrap();
        let result = section.remove_pin_entry("pkgs-nonexistent");
        assert!(result.is_err());
    }

    // ========================================================================
    // PACKAGE TESTS
    // ========================================================================

    #[test]
    fn test_package_in_pin_exists_with_existing_package() {
        let section = parse_overlay_section(PINS_CONTENT).unwrap();
        let result = section.package_in_pin_exists("pkgs-abc1234", "git");
        assert!(result);
    }

    #[test]
    fn test_package_in_pin_exists_with_nonexistent_package() {
        let section = parse_overlay_section(PINS_CONTENT).unwrap();
        let result = section.package_in_pin_exists("pkgs-abc1234", "python");
        assert!(!result);
    }

    #[test]
    fn test_add_package_to_pin() {
        let mut section = parse_overlay_section(PINS_CONTENT).unwrap();
        section
            .add_package_to_pin("pkgs-abc1234", "wget", "wget@1.21.0")
            .unwrap();

        assert!(section.package_in_pin_exists("pkgs-abc1234", "git")); // Existing packages still there
        assert!(section.package_in_pin_exists("pkgs-abc1234", "curl"));
        assert!(section.package_in_pin_exists("pkgs-abc1234", "wget")); // New package added
    }

    #[test]
    fn test_remove_package_from_pin() {
        let mut section = parse_overlay_section(PINS_CONTENT).unwrap();
        section
            .remove_package_from_pin("pkgs-abc1234", "git")
            .unwrap();
        assert!(!section.package_in_pin_exists("pkgs-abc1234", "git"));
        assert!(section.package_in_pin_exists("pkgs-abc1234", "curl"));
    }

    #[test]
    fn test_remove_nonexistent_package_from_pin() {
        let mut section = parse_overlay_section(PINS_CONTENT).unwrap();
        let result = section.remove_package_from_pin("pkgs-abc1234", "nonexistent@1.0");
        assert!(result.is_err());
    }

    // ========================================================================
    // INDENT TESTS
    // ========================================================================

    #[test]
    fn test_indent_consistency() {
        let section = parse_packages_section(PROFILE_CONTENT).unwrap();
        let result = section.add_package(PROFILE_CONTENT, "test", None);
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
    use flk::flake::interfaces::profiles::{EnvVar, FlakeConfig, Package, Profile};

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
