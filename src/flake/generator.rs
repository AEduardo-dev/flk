use anyhow::{Ok, Result};

const ROOT_FLAKE_TEMPLATE: &str = include_str!("../../templates/flake.nix");
const HELPER_TEMPLATE: &str = include_str!("../../templates/default.nix");
const IMPORTER_TEMPLATE: &str = include_str!("../../templates/profiles/default.nix");

const GENERIC_TEMPLATE: &str = include_str!("../../templates/profiles/base.nix");
const RUST_TEMPLATE: &str = include_str!("../../templates/profiles/rust.nix");
const PYTHON_TEMPLATE: &str = include_str!("../../templates/profiles/python.nix");
const NODE_TEMPLATE: &str = include_str!("../../templates/profiles/node.nix");
const GO_TEMPLATE: &str = include_str!("../../templates/profiles/go.nix");

pub fn generate_root_flake() -> Result<String> {
    Ok(ROOT_FLAKE_TEMPLATE.to_string())
}

pub fn generate_helper_module() -> Result<String> {
    Ok(HELPER_TEMPLATE.to_string())
}

pub fn generate_importer_module() -> Result<String> {
    Ok(IMPORTER_TEMPLATE.to_string())
}

pub fn generate_flake(project_type: &str) -> Result<String> {
    let template = match project_type {
        "rust" => RUST_TEMPLATE,
        "python" => PYTHON_TEMPLATE,
        "node" => NODE_TEMPLATE,
        "go" => GO_TEMPLATE,
        _ => GENERIC_TEMPLATE,
    };

    Ok(template.to_string())
}

pub fn generate_hooks() -> String {
    let hook_script = include_str!("../helpers/hooks.sh");
    hook_script.to_string()
}
