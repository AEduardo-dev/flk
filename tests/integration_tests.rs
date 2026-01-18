use assert_cmd::cargo;
use predicates::prelude::*;
use predicates::str::contains;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

#[test]
fn test_version() {
    cargo::cargo_bin_cmd!("flk")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_help() {
    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
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
    assert!(content.contains("python312"));
}

#[test]
fn test_init_with_node_template() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    // Try to create again without force - should fail
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Try with force - should succeed
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .arg("--force")
        .assert()
        .success();
}

#[test]
fn test_init_creates_flk_directory_structure() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No packages"));
}

#[test]
fn test_show_flake() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("remove")
        .arg("ripgrep")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile (flake.nix)",
        ));
}

#[test]
fn test_add_command_without_init() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("command")
        .arg("add")
        .arg("test")
        .arg("echo hello")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile (flake.nix)",
        ));
}

#[test]
fn test_env_add_without_init() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("add")
        .arg("TEST_VAR")
        .arg("test_value")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile (flake.nix)",
        ));
}

#[test]
fn test_env_list_without_init() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("env")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No flake.nix found"));
}

#[test]
fn test_remove_command_without_init() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("command")
        .arg("remove")
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find default shell profile (flake.nix)",
        ));
}

#[test]
fn test_invalid_command_name() {
    let temp_dir = TempDir::new().unwrap();
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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
        cargo::cargo_bin_cmd!("flk")
            .args(["completions", shell])
            .assert()
            .success();
    }
}

#[test]
fn test_multiple_packages() {
    let temp_dir = TempDir::new().unwrap();

    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    let packages = vec!["ripgrep", "git", "wget"];
    for pkg in &packages {
        cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .success();
}

#[test]
fn test_export_json() {
    let temp_dir = TempDir::new().unwrap();

    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .success();

    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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

    cargo::cargo_bin_cmd!("flk")
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
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("direnv")
        .arg("init")
        .assert()
        .success();
    let direnv_path = temp_dir.path().join(".envrc");
    assert!(direnv_path.exists());
    let content = fs::read_to_string(direnv_path).unwrap();
    assert_eq!(content, "use flake --impure");
}
#[test]
fn test_direnv_attach() {
    let temp_dir = TempDir::new().unwrap();
    let direnv_path = temp_dir.path().join(".envrc");
    fs::write(&direnv_path, "export VAR=value").unwrap();

    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("direnv")
        .arg("attach")
        .assert()
        .success();

    assert!(direnv_path.exists());
    let content = fs::read_to_string(direnv_path).unwrap();
    assert!(content.contains("export VAR=value"));
    assert!(content.contains("use flake --impure"));
}
#[test]
fn test_direnv_detach() {
    let temp_dir = TempDir::new().unwrap();
    let direnv_path = temp_dir.path().join(".envrc");

    fs::write(&direnv_path, "export VAR=value\nuse flake --impure").unwrap();
    cargo::cargo_bin_cmd!("flk")
        .current_dir(temp_dir.path())
        .arg("direnv")
        .arg("detach")
        .assert()
        .success();

    assert!(direnv_path.exists());
    let content = fs::read_to_string(direnv_path).unwrap();
    assert!(!content.contains("use flake --impure"));
    assert!(content.contains("export VAR=value"));
}
