use anyhow::Result;

const GENERIC_TEMPLATE: &str = include_str!("../../templates/default_flake.nix");
const RUST_TEMPLATE: &str = include_str!("../../templates/rust_flake.nix");
const PYTHON_TEMPLATE: &str = include_str!("../../templates/python_flake.nix");
const NODE_TEMPLATE: &str = include_str!("../../templates/node_flake.nix");
const GO_TEMPLATE: &str = include_str!("../../templates/go_flake.nix");

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
    let hook_script = include_str!("../hooks/hooks.sh");
    hook_script.to_string()
}
