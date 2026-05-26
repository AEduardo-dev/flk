use std::path::{Path, PathBuf};

/// Watch list used by both the activate cache and the shell hook.
///
/// The slim layout has no `.flk/default.nix` or `.flk/overlays.nix` — those
/// live in the remote `flk.lib.mkProject` driver and bumps come in through
/// `flake.lock`. The legacy layout still tracks them locally.
const SLIM_INPUTS: &[&str] = &[
    "flake.nix",
    "flake.lock",
    ".flk/config.nix",
    ".flk/pins.nix",
];

const LEGACY_INPUTS: &[&str] = &[
    "flake.nix",
    "flake.lock",
    ".flk/default.nix",
    ".flk/pins.nix",
    ".flk/overlays.nix",
];

fn is_slim_layout() -> bool {
    Path::new(".flk/config.nix").exists()
}

fn base_inputs() -> &'static [&'static str] {
    if is_slim_layout() {
        SLIM_INPUTS
    } else {
        LEGACY_INPUTS
    }
}

pub(crate) fn profile_cache_inputs(profile: &str) -> Vec<PathBuf> {
    let mut paths = base_inputs().iter().map(PathBuf::from).collect::<Vec<_>>();
    paths.push(Path::new(".flk/profiles").join(format!("{profile}.nix")));
    paths
}

/// Generate a space-separated list of quoted cache-input paths for shell `for` loops.
///
/// `profile_expr` should be a shell variable reference (e.g. `"$profile"`) so it is
/// expanded at runtime by the target shell. Layout detection happens once at hook
/// emission time — the resulting shell snippet is layout-fixed.
pub(crate) fn profile_cache_hook_inputs(profile_expr: &str) -> String {
    base_inputs()
        .iter()
        .map(|path| format!("\"{path}\""))
        .chain(std::iter::once(format!(
            "\".flk/profiles/{profile_expr}.nix\""
        )))
        .collect::<Vec<_>>()
        .join(" ")
}
