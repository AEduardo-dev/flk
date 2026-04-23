use std::path::{Path, PathBuf};

pub(crate) const PROFILE_CACHE_INPUTS: [&str; 5] = [
    "flake.nix",
    "flake.lock",
    ".flk/default.nix",
    ".flk/pins.nix",
    ".flk/overlays.nix",
];

pub(crate) fn profile_cache_inputs(profile: &str) -> Vec<PathBuf> {
    let mut paths = PROFILE_CACHE_INPUTS
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    paths.push(Path::new(".flk/profiles").join(format!("{profile}.nix")));
    paths
}

pub(crate) fn profile_cache_hook_inputs(profile_expr: &str) -> String {
    PROFILE_CACHE_INPUTS
        .iter()
        .map(|path| format!("\"{path}\""))
        .chain(std::iter::once(format!(
            "\".flk/profiles/{profile_expr}.nix\""
        )))
        .collect::<Vec<_>>()
        .join(" ")
}
