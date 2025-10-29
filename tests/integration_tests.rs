use assert_cmd::Command;
use predicates::prelude::*;
use predicates::str::contains;
use std::fs;
use tempfile::TempDir;

/// Helper to create a test command in a temporary directory
fn flk_command() -> (Command, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("flk").unwrap();
    cmd.current_dir(temp_dir.path());
    (cmd, temp_dir)
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin("flk").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("flk").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A CLI tool for managing flake.nix",
        ));
}

#[test]
fn test_init_without_template() {
    let (mut cmd, temp_dir) = flk_command();

    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created flake.nix successfully!"));

    // Check that flake.nix was created
    let flake_path = temp_dir.path().join("flake.nix");
    assert!(flake_path.exists());

    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("description = \"Development environment managed by flk\""));
}

#[test]
fn test_init_with_rust_template() {
    let (mut cmd, temp_dir) = flk_command();

    cmd.arg("init")
        .arg("--template")
        .arg("rust")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Initializing flake for rust project",
        ));

    let flake_path = temp_dir.path().join("flake.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Rust development environment"));
    assert!(content.contains("rust-bin.stable.latest.default"));
}

#[test]
fn test_init_with_python_template() {
    let (mut cmd, temp_dir) = flk_command();

    cmd.arg("init")
        .arg("--template")
        .arg("python")
        .assert()
        .success();

    let flake_path = temp_dir.path().join("flake.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Python development environment"));
    assert!(content.contains("python311"));
}

#[test]
fn test_init_force_overwrite() {
    let (mut cmd, temp_dir) = flk_command();

    // Create initial flake
    cmd.arg("init").assert().success();

    // Try to create again without force - should fail
    let mut cmd2 = Command::cargo_bin("flk").unwrap();
    cmd2.current_dir(temp_dir.path())
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));

    // Try with force - should succeed
    let mut cmd3 = Command::cargo_bin("flk").unwrap();
    cmd3.current_dir(temp_dir.path())
        .arg("init")
        .arg("--force")
        .assert()
        .success();
}

#[test]
fn test_list_empty_flake() {
    let (mut init_cmd, temp_dir) = flk_command();
    init_cmd.arg("init").assert().success();

    let mut list_cmd = Command::cargo_bin("flk").unwrap();
    list_cmd
        .current_dir(temp_dir.path())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Installed Packages"));
}

#[test]
fn test_show_flake() {
    let (mut init_cmd, temp_dir) = flk_command();
    init_cmd.arg("init").assert().success();

    let mut show_cmd = Command::cargo_bin("flk").unwrap();
    show_cmd
        .current_dir(temp_dir.path())
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("Flake Configuration"))
        .stdout(predicate::str::contains("nixpkgs"));
}

#[test]
fn test_add_package_without_init() {
    let (mut cmd, _temp_dir) = flk_command();

    cmd.arg("add")
        .arg("ripgrep")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No flake.nix found"));
}

#[test]
fn test_remove_package_without_init() {
    let (mut cmd, _temp_dir) = flk_command();

    cmd.arg("remove")
        .arg("ripgrep")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No flake.nix found"));
}

#[test]
fn test_add_command_without_init() {
    let (mut cmd, _temp_dir) = flk_command();

    cmd.arg("add-command")
        .arg("test")
        .arg("echo hello")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No flake.nix found"));
}

#[test]
fn test_env_add_without_init() {
    let (mut cmd, _temp_dir) = flk_command();

    cmd.arg("env")
        .arg("add")
        .arg("TEST_VAR")
        .arg("test_value")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No flake.nix found"));
}

#[test]
fn test_env_list_without_init() {
    let (mut cmd, _temp_dir) = flk_command();

    cmd.arg("env")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No flake.nix found"));
}

#[test]
fn test_remove_command_without_init() {
    let (mut cmd, _temp_dir) = flk_command();

    cmd.arg("remove-command")
        .arg("test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No flake.nix found"));
}

#[test]
fn test_invalid_command_name() {
    let (mut init_cmd, temp_dir) = flk_command();
    init_cmd.arg("init").assert().success();

    let mut cmd = Command::cargo_bin("flk").unwrap();
    cmd.current_dir(temp_dir.path())
        .arg("add-command")
        .arg("\"-invalid-name\"")
        .arg("echo test")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid command name"));
}

#[test]
fn test_env_add_invalid_name() {
    let (mut init_cmd, temp_dir) = flk_command();
    init_cmd.arg("init").assert().success();

    let mut cmd = Command::cargo_bin("flk").unwrap();
    cmd.current_dir(temp_dir.path())
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
    let (mut cmd, temp_dir) = flk_command();

    // Create a Cargo.toml to trigger Rust detection
    fs::write(
        temp_dir.path().join("Cargo.toml"),
        "[package]\nname = \"test\"",
    )
    .unwrap();

    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Rust project"));

    let flake_path = temp_dir.path().join("flake.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Rust development environment"));
}

#[test]
fn test_auto_detect_python_project() {
    let (mut cmd, temp_dir) = flk_command();

    // Create a pyproject.toml to trigger Python detection
    fs::write(temp_dir.path().join("pyproject.toml"), "[tool.poetry]").unwrap();

    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Python project"));

    let flake_path = temp_dir.path().join("flake.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Python development environment"));
}

#[test]
fn test_auto_detect_node_project() {
    let (mut cmd, temp_dir) = flk_command();

    // Create a package.json to trigger Node.js detection
    fs::write(temp_dir.path().join("package.json"), "{}").unwrap();

    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Node.js project"));

    let flake_path = temp_dir.path().join("flake.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Node.js development environment"));
}

#[test]
fn test_auto_detect_go_project() {
    let (mut cmd, temp_dir) = flk_command();

    // Create a go.mod to trigger Go detection
    fs::write(temp_dir.path().join("go.mod"), "module test").unwrap();

    cmd.arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Detected Go project"));

    let flake_path = temp_dir.path().join("flake.nix");
    let content = fs::read_to_string(flake_path).unwrap();
    assert!(content.contains("Go development environment"));
}

#[test]
fn test_completions_prints_bash_script() {
    let mut cmd = Command::cargo_bin("flk").unwrap();
    cmd.args(["completions", "bash"])
        .assert()
        .success()
        .stdout(contains("_flk()"));
}

#[test]
fn test_completions_install_creates_file() {
    let temp = tempfile::tempdir().unwrap();
    std::env::set_var("HOME", temp.path()); // redirect install location

    let mut cmd = Command::cargo_bin("flk").unwrap();
    cmd.args(["completions", "--install", "zsh"])
        .assert()
        .success();

    let installed = temp.path().join(".zsh/completions/_flk");
    assert!(
        installed.exists(),
        "Expected completion file at {:?}",
        installed
    );
}
