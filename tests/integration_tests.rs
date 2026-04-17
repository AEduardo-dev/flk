use assert_cmd::cargo;
use predicates::prelude::*;
use predicates::str::contains;
use std::fs;
use tempfile::TempDir;

/// Create a `flk` command with a clean environment (no `FLK_FLAKE_REF` leaking in).
fn flk_cmd() -> assert_cmd::Command {
    let mut cmd = cargo::cargo_bin_cmd!("flk");
    cmd.env_remove("FLK_FLAKE_REF");
    cmd
}

#[test]
fn test_version() {
    flk_cmd()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_help() {
    flk_cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A CLI tool for managing flake.nix devShell environments",
        ));
}

#[test]
fn test_init_without_template() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Created flk environment successfully!",
        ));

    // Check that . flk directory structure was created
    let flk_dir = temp_dir.path().join(".flk");
    assert!(flk_dir.exists());

    let profiles_dir = temp_dir.path().join(".flk/profiles");
    assert!(profiles_dir.exists());

    // Check that a profile file was created
    let profile_path = temp_dir.path().join(".flk/profiles/generic.nix");
    assert!(profile_path.exists());

    let content = fs::read_to_string(profile_path).unwrap();
    assert!(content.contains("description = \"Generic Development Environment\""));
}

#[test]
fn test_init_with_rust_template() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--template")
        .arg("rust")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Initializing flake for rust project",
        ));

    let flake_path = temp_dir.path().join(".flk/profiles/rust.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Rust development environment"));
    assert!(content.contains("rust-bin.stable.latest.default"));
}

#[test]
fn test_init_with_python_template() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--template")
        .arg("python")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Initializing flake for python project",
        ));

    let flake_path = temp_dir.path().join(".flk/profiles/python.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Python development environment"));
    assert!(content.contains("python3"));
}

#[test]
fn test_init_with_node_template() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--template")
        .arg("node")
        .assert()
        .success();

    let flake_path = temp_dir.path().join(".flk/profiles/node.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Node.js development environment"));
}

#[test]
fn test_init_with_go_template() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--template")
        .arg("go")
        .assert()
        .success();

    let flake_path = temp_dir.path().join(".flk/profiles/go.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Go development environment"));
}

#[test]
fn test_init_force_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Try to create again without force - should fail
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Try with force - should succeed
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--force")
        .assert()
        .success();
}

#[test]
fn test_init_creates_flk_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Verify directory structure
    assert!(temp_dir.path().join(".flk").exists());
    assert!(temp_dir.path().join(".flk/profiles").exists());
    assert!(temp_dir.path().join("flake.nix").exists());
}

#[test]
fn test_list_empty_flake() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No packages"));
}

#[test]
fn test_show_flake() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("Flake Configuration"))
        .stdout(predicate::str::contains("nixpkgs"));
}

#[test]
fn test_add_package_without_init() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("add")
        .arg("ripgrep")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile",
        ));
}

#[test]
fn test_remove_package_without_init() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("remove")
        .arg("ripgrep")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile",
        ));
}

#[test]
fn test_add_command_without_init() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("command")
        .arg("add")
        .arg("test")
        .arg("echo hello")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile",
        ));
}

#[test]
fn test_env_add_without_init() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("add")
        .arg("TEST_VAR")
        .arg("test_value")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile",
        ));
}

#[test]
fn test_env_list_without_init() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile",
        ));
}

#[test]
fn test_remove_command_without_init() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("command")
        .arg("remove")
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile",
        ));
}

#[test]
fn test_invalid_command_name() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("command")
        .arg("add")
        .arg("\"-invalid-name\"")
        .arg("echo test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid command name"));
}

#[test]
fn test_env_add_invalid_name() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("add")
        .arg("123INVALID")
        .arg("value")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Invalid environment variable name",
        ));
}

#[test]
fn test_auto_detect_rust_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create a Cargo.toml to trigger Rust detection
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname = \"test\"",
    )
    .unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Rust project"));

    let flake_path = temp_dir.path().join(".flk/profiles/rust.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Rust development environment"));
}

#[test]
fn test_auto_detect_python_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create a pyproject.toml to trigger Python detection
    fs::write(temp_dir.path().join("pyproject.toml"), "[tool.poetry]").unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Python project"));

    let flake_path = temp_dir.path().join(".flk/profiles/python.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Python development environment"));
}

#[test]
fn test_auto_detect_node_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create a package.json to trigger Node.js detection
    fs::write(temp_dir.path().join("package.json"), "{}").unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Node.js project"));

    let flake_path = temp_dir.path().join(".flk/profiles/node.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Node.js development environment"));
}

#[test]
fn test_auto_detect_go_project() {
    let temp_dir = TempDir::new().unwrap();

    // Create a go.mod to trigger Go detection
    fs::write(temp_dir.path().join("go.mod"), "module test").unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Go project"));

    let flake_path = temp_dir.path().join(".flk/profiles/go.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Go development environment"));
}

#[test]
fn test_completions_prints_bash_script() {
    flk_cmd()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(contains("_flk()"));
}

#[test]
fn test_completions_install_creates_file() {
    let temp = tempfile::tempdir().unwrap();
    unsafe {
        std::env::set_var("HOME", temp.path());
    }

    flk_cmd()
        .args(["completions", "--install", "zsh"])
        .assert()
        .success();

    let installed = temp.path().join(".zsh/completions/_flk");
    assert!(
        installed.exists(),
        "Expected completion file at {:?}",
        installed
    );
}

#[test]
fn test_completions_all_shells() {
    let shells = vec!["bash", "zsh", "fish", "powershell", "elvish"];

    for shell in shells {
        flk_cmd().args(["completions", shell]).assert().success();
    }
}

#[test]
fn test_multiple_packages() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    let packages = vec!["ripgrep", "git", "wget"];
    for pkg in &packages {
        flk_cmd()
            .current_dir(temp_dir.path())
            .arg("add")
            .arg(pkg)
            .assert()
            .success();
    }

    let profile_path = temp_dir.path().join(".flk/profiles/generic.nix");
    let content = fs::read_to_string(&profile_path).unwrap();
    for pkg in &packages {
        assert!(content.contains(pkg));
    }

    // List should show all packages
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_profile_add_list_set_default_remove() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("add")
        .arg("rust")
        .arg("--template")
        .arg("rust")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("generic"))
        .stdout(predicate::str::contains("rust"));

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("set-default")
        .arg("rust")
        .assert()
        .success();

    let default_content = fs::read_to_string(temp_dir.path().join(".flk/default.nix")).unwrap();
    assert!(default_content.contains("defaultShell = \"rust\";"));

    // Cannot remove rust while it's the default
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("remove")
        .arg("rust")
        .assert()
        .failure()
        .stderr(predicate::str::contains("currently set as the default"));

    // Switch back to generic, then remove rust
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("set-default")
        .arg("generic")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("remove")
        .arg("rust")
        .assert()
        .success();

    assert!(!temp_dir.path().join(".flk/profiles/rust.nix").exists());
}

#[test]
fn test_profile_name_validation() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Invalid names should fail
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("add")
        .arg("../bad")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid profile name"));

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("add")
        .arg("has spaces")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid profile name"));
}

#[test]
fn test_path_traversal_prevention() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Path traversal via --profile flag should be rejected
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("add")
        .arg("ripgrep")
        .arg("--profile")
        .arg("../../../tmp/malicious")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid profile name"));

    // Path traversal via env command should also be rejected
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("--profile")
        .arg("../secret")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid profile name"));

    // Path traversal via profile remove should be rejected
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("remove")
        .arg("../../../etc/passwd")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid profile name"));

    // Path traversal via profile set-default should be rejected
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("set-default")
        .arg("../../../tmp/malicious")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid profile name"));
}

#[test]
fn test_add_package_to_specific_profile() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Create a second profile
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("add")
        .arg("rust")
        .arg("--template")
        .arg("rust")
        .assert()
        .success();

    // Add package to the rust profile specifically
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("add")
        .arg("ripgrep")
        .arg("--profile")
        .arg("rust")
        .assert()
        .success();

    // Verify package is in rust profile
    let rust_profile = fs::read_to_string(temp_dir.path().join(".flk/profiles/rust.nix")).unwrap();
    assert!(rust_profile.contains("ripgrep"));

    // Verify package is NOT in generic profile
    let generic_profile =
        fs::read_to_string(temp_dir.path().join(".flk/profiles/generic.nix")).unwrap();
    assert!(!generic_profile.contains("ripgrep"));
}

#[test]
fn test_env_operations_on_specific_profile() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Create a second profile
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("profile")
        .arg("add")
        .arg("dev")
        .arg("--template")
        .arg("base")
        .assert()
        .success();

    // Add env var to dev profile
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("--profile")
        .arg("dev")
        .arg("add")
        .arg("MY_VAR")
        .arg("my_value")
        .assert()
        .success();

    // Verify env var is in dev profile
    let dev_profile = fs::read_to_string(temp_dir.path().join(".flk/profiles/dev.nix")).unwrap();
    assert!(dev_profile.contains("MY_VAR"));

    // List env vars on specific profile
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("--profile")
        .arg("dev")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("MY_VAR"));
}

#[test]
fn test_export_json() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("export")
        .arg("--format")
        .arg("json")
        .assert()
        .success();

    let json_path = temp_dir.path().join("flake.json");
    assert!(json_path.exists());

    let json_content = fs::read_to_string(json_path).unwrap();
    assert!(json_content.contains("profiles"));
}

#[test]
fn test_profile_directory_isolation() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--template")
        .arg("rust")
        .assert()
        .success();

    // Check that only rust profile exists
    let profiles_dir = temp_dir.path().join(".flk/profiles");
    let entries: Vec<_> = fs::read_dir(&profiles_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "nix").unwrap_or(false))
        .collect();

    // Should have rust. nix and possibly default.nix
    assert!(entries.len() >= 1);
    assert!(profiles_dir.join("rust.nix").exists());
}

#[test]
fn test_flake_nix_exists_at_root() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Root flake.nix should exist
    assert!(temp_dir.path().join("flake.nix").exists());
}

#[test]
fn test_dendritic_structure_complete() {
    let temp_dir = TempDir::new().unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Verify complete dendritic structure
    assert!(temp_dir.path().join("flake.nix").exists());
    assert!(temp_dir.path().join(".flk").exists());
    assert!(temp_dir.path().join(".flk/profiles").exists());

    // Check for helper files that might be generated
    let flk_dir = temp_dir.path().join(".flk");
    assert!(flk_dir.is_dir());
}

// Direnv integration tests
#[test]
fn test_direnv_init() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("direnv")
        .arg("init")
        .assert()
        .success();
    let direnv_path = temp_dir.path().join(".envrc");
    assert!(direnv_path.exists());
    let content = fs::read_to_string(direnv_path).unwrap();
    assert_eq!(content, "# Watch flk config files so nix-direnv re-evaluates on changes\nwatch_file .flk/default.nix\nwatch_file .flk/pins.nix\nwatch_file .flk/overlays.nix\nfor f in .flk/profiles/*.nix; do watch_file \"$f\"; done\n\nuse flake \"${FLK_PROFILE:-.#}\" --impure");
}
#[test]
fn test_direnv_attach() {
    let temp_dir = TempDir::new().unwrap();
    let direnv_path = temp_dir.path().join(".envrc");
    fs::write(&direnv_path, "export VAR=value").unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("direnv")
        .arg("attach")
        .assert()
        .success();

    assert!(direnv_path.exists());
    let content = fs::read_to_string(direnv_path).unwrap();
    assert!(content.contains("export VAR=value"));
    assert!(content.contains("use flake \"${FLK_PROFILE:-.#}\" --impure"));
}
#[test]
fn test_direnv_detach() {
    let temp_dir = TempDir::new().unwrap();

    // Init to get full directive with watch_file lines, then add user content
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("direnv")
        .arg("init")
        .assert()
        .success();

    let direnv_path = temp_dir.path().join(".envrc");
    let mut content = fs::read_to_string(&direnv_path).unwrap();
    content.push_str("\nexport VAR=value\nwatch_file my_file.txt\n");
    fs::write(&direnv_path, &content).unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("direnv")
        .arg("detach")
        .assert()
        .success();

    assert!(direnv_path.exists());
    let content = fs::read_to_string(direnv_path).unwrap();
    // All flk directives removed
    assert!(!content.contains("use flake"));
    assert!(!content.contains("watch_file .flk/"));
    assert!(!content.contains("for f in .flk/profiles"));
    assert!(!content.contains("# Watch flk config files"));
    // User content preserved
    assert!(content.contains("export VAR=value"));
    assert!(content.contains("watch_file my_file.txt"));
}

// --- command.rs success & error paths ---

#[test]
fn test_command_add_list_remove() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["command", "add", "greet", "echo hello"])
        .assert()
        .success()
        .stdout(contains("Command 'greet' added successfully"));

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["command", "list"])
        .assert()
        .success()
        .stdout(contains("greet"));

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["command", "remove", "greet"])
        .assert()
        .success()
        .stdout(contains("Command 'greet' removed successfully"));

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["command", "list"])
        .assert()
        .success()
        .stdout(contains("No commands found"));
}

#[test]
fn test_command_add_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args([
            "command",
            "--profile",
            "nonexistent",
            "add",
            "greet",
            "echo hi",
        ])
        .assert()
        .failure()
        .stderr(contains("Failed to read profile file"));
}

#[test]
fn test_command_remove_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["command", "--profile", "nonexistent", "remove", "greet"])
        .assert()
        .failure()
        .stderr(contains("Profile file"));
}

#[test]
fn test_command_remove_nonexistent_command() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["command", "remove", "nonexistent_cmd"])
        .assert()
        .failure()
        .stderr(contains("not found in profile"));
}

#[test]
fn test_command_list_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["command", "--profile", "nonexistent", "list"])
        .assert()
        .failure()
        .stderr(contains("Failed to read profile file"));
}

// --- env.rs success & error paths ---

#[test]
fn test_env_add_and_remove() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["env", "add", "MY_VAR", "my_value"])
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["env", "remove", "MY_VAR"])
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["env", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("MY_VAR").not());
}

#[test]
fn test_env_add_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["env", "--profile", "nonexistent", "add", "MY_VAR", "value"])
        .assert()
        .failure()
        .stderr(contains("Failed to read profile file"));
}

#[test]
fn test_env_remove_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["env", "--profile", "nonexistent", "remove", "MY_VAR"])
        .assert()
        .failure()
        .stderr(contains("Failed to read profile file"));
}

#[test]
fn test_env_list_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["env", "--profile", "nonexistent", "list"])
        .assert()
        .failure()
        .stderr(contains("Failed to read profile file"));
}

// --- remove.rs & list.rs error paths ---

#[test]
fn test_remove_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["remove", "ripgrep", "--profile", "nonexistent"])
        .assert()
        .failure()
        .stderr(contains("Failed to read profile file"));
}

#[test]
fn test_list_nonexistent_profile() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    flk_cmd()
        .current_dir(temp_dir.path())
        .args(["list", "--profile", "nonexistent"])
        .assert()
        .failure()
        .stderr(contains("Failed to read profile file"));
}

// --- utils.rs resolve_profile paths ---

#[test]
fn test_resolve_profile_flk_flake_ref_empty_fallback() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // FLK_FLAKE_REF=".#" normalizes to None, triggering fallback to default profile
    let mut cmd = cargo::cargo_bin_cmd!("flk");
    cmd.env("FLK_FLAKE_REF", ".#");
    cmd.current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_resolve_profile_flk_flake_ref_valid() {
    let temp_dir = TempDir::new().unwrap();
    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // FLK_FLAKE_REF=".#generic" normalizes to "generic", using it as the profile
    let mut cmd = cargo::cargo_bin_cmd!("flk");
    cmd.env("FLK_FLAKE_REF", ".#generic");
    cmd.current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_no_profiles_available() {
    let temp_dir = TempDir::new().unwrap();

    // Create minimal .flk structure with no profiles
    fs::create_dir_all(temp_dir.path().join(".flk/profiles")).unwrap();
    fs::write(temp_dir.path().join(".flk/default.nix"), "{ }").unwrap();

    flk_cmd()
        .current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .failure()
        .stderr(contains("No profiles found"));
}
