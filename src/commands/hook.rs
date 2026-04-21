//! # Hook Command Handler
//!
//! Generate shell hooks for bash, zsh, and fish that enable the
//! `refresh` and `switch` commands for hot-reloading environments.

use anyhow::Result;
use clap::ValueEnum;

/// Supported shell types for hook generation.
#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum HookShell {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
}

/// Generate and print the shell hook for the specified shell.
pub fn run_hook(shell: HookShell) -> Result<()> {
    match shell {
        HookShell::Bash | HookShell::Zsh => print_bash_like(),
        HookShell::Fish => print_fish(),
    }
    Ok(())
}

fn print_bash_like() {
    println!(
        r#"# flk hook: refresh/switch for direnv + nix develop
_flk_use_direnv() {{ command -v direnv >/dev/null 2>&1 && [ -f .envrc ]; }}
_flk_valid_profile() {{ [[ "$1" =~ ^[a-zA-Z0-9_-]+$ ]]; }}
_flk_profile_path() {{ printf '.flk/.nix-profile-%s' "$1"; }}
_flk_profile_stamp() {{ printf '.flk/.nix-profile-%s.stamp' "$1"; }}
_flk_profile_is_fresh() {{
  local profile="$1"
  local profile_path="$(_flk_profile_path "$profile")"
  local stamp_path="$(_flk_profile_stamp "$profile")"
  local f
  [ -e "$profile_path" ] && [ -e "$stamp_path" ] || return 1
  for f in "flake.nix" "flake.lock" ".flk/default.nix" ".flk/pins.nix" ".flk/overlays.nix" ".flk/profiles/$profile.nix"; do
    [ -e "$f" ] || continue
    [ "$f" -nt "$stamp_path" ] && return 1
  done
  return 0
}}
_flk_exec_nix_develop() {{
  local ref="$1"
  local profile="$2"
  local flk_shell="$3"
  local profile_path="$(_flk_profile_path "$profile")"
  local stamp_path="$(_flk_profile_stamp "$profile")"
  if _flk_profile_is_fresh "$profile"; then
    exec nix develop "$profile_path" --impure -c "$flk_shell"
  else
    exec env FLK_REF="$ref" FLK_PROFILE_PATH="$profile_path" FLK_SHELL_CMD="$flk_shell" FLK_PROFILE_STAMP="$stamp_path" nix develop "$ref" --impure --profile "$profile_path" -c /bin/sh -c '
      if [ -e "$FLK_PROFILE_PATH" ]; then
        mkdir -p "$(dirname "$FLK_PROFILE_STAMP")" &&
        touch "$FLK_PROFILE_STAMP" ||
        exit 1
      fi
      exec "$FLK_SHELL_CMD"
    '
  fi
}}

refresh() {{
  # Fallback order: FLK_FLAKE_REF -> FLK_PROFILE -> .#default
  local _flk_ref="${{FLK_FLAKE_REF:-${{FLK_PROFILE:-.#default}}}}"
  local _flk_profile_name="${{_flk_ref##*.#}}"
  if ! _flk_valid_profile "$_flk_profile_name"; then
    printf 'invalid profile name: %s\n' "$_flk_profile_name" >&2
    return 1
  fi
  export FLK_FLAKE_REF="$_flk_ref"
  export FLK_PROFILE="$_flk_ref"
  if _flk_use_direnv; then
    direnv reload
  else
    _flk_exec_nix_develop "$_flk_ref" "$_flk_profile_name" "${{SHELL:-/bin/sh}}"
  fi
}}

switch() {{
  local profile="$1"
  if [ -z "$profile" ]; then
    printf 'usage: switch <profile>\n' >&2
    return 1
  fi
  if ! _flk_valid_profile "$profile"; then
    printf 'invalid profile name: %s\n' "$profile" >&2
    return 1
  fi
  if _flk_use_direnv; then
      export FLK_FLAKE_REF=".#$profile"
      export FLK_PROFILE=".#$profile"
      direnv reload
    else
    export FLK_FLAKE_REF=".#$profile"
    export FLK_PROFILE=".#$profile"
    _flk_exec_nix_develop ".#$profile" "$profile" "${{SHELL:-/bin/sh}}"
  fi
}}
"#
    );
}

fn print_fish() {
    println!(
        r#"# flk hook: refresh/switch for direnv + nix develop (fish)
function _flk_use_direnv
  type -q direnv; and test -f .envrc
end

function _flk_valid_profile
  string match -qr '^[a-zA-Z0-9_-]+$' -- $argv[1]
end

function _flk_profile_path
  printf '.flk/.nix-profile-%s' $argv[1]
end

function _flk_profile_stamp
  printf '.flk/.nix-profile-%s.stamp' $argv[1]
end

function _flk_profile_is_fresh
  set profile $argv[1]
  set profile_path (_flk_profile_path "$profile")
  set stamp_path (_flk_profile_stamp "$profile")
  test -e "$profile_path"; and test -e "$stamp_path"; or return 1
  for f in "flake.nix" "flake.lock" ".flk/default.nix" ".flk/pins.nix" ".flk/overlays.nix" ".flk/profiles/$profile.nix"
    test -e "$f"; or continue
    test "$f" -nt "$stamp_path"; and return 1
  end
  return 0
end

function _flk_exec_nix_develop
  set ref $argv[1]
  set profile $argv[2]
  set flk_shell $argv[3]
  set profile_path (_flk_profile_path "$profile")
  set stamp_path (_flk_profile_stamp "$profile")
  if _flk_profile_is_fresh "$profile"
    exec nix develop "$profile_path" --impure -c "$flk_shell"
  else
    exec env FLK_REF="$ref" FLK_PROFILE_PATH="$profile_path" FLK_SHELL_CMD="$flk_shell" FLK_PROFILE_STAMP="$stamp_path" nix develop "$ref" --impure --profile "$profile_path" -c /bin/sh -c '
      if [ -e "$FLK_PROFILE_PATH" ]; then
        mkdir -p "$(dirname "$FLK_PROFILE_STAMP")" &&
        touch "$FLK_PROFILE_STAMP" ||
        exit 1
      fi
      exec "$FLK_SHELL_CMD"
    '
  end
end

function refresh --description "Reload env (direnv if present, else nix develop)"
  # Fallback order: FLK_FLAKE_REF -> FLK_PROFILE -> .#default
  set -l flk_ref (test -n "$FLK_FLAKE_REF"; and echo "$FLK_FLAKE_REF"; or test -n "$FLK_PROFILE"; and echo "$FLK_PROFILE"; or echo ".#default")
  set -l profile_name (string replace -r '.*\\.#' '' "$flk_ref")
  if not _flk_valid_profile "$profile_name"
    echo "invalid profile name: $profile_name" 1>&2
    return 1
  end
  if _flk_use_direnv
    set -lx FLK_FLAKE_REF "$flk_ref"
    set -lx FLK_PROFILE "$flk_ref"
    direnv reload
  else
    set -l flk_shell (test -n "$SHELL"; and echo "$SHELL"; or echo "/bin/sh")
    _flk_exec_nix_develop "$flk_ref" "$profile_name" "$flk_shell"
  end
end

function switch --description "Switch profile and reload"
  if test (count $argv) -lt 1
    echo "usage: switch <profile>" 1>&2
    return 1
  end
  set profile $argv[1]
  if not _flk_valid_profile "$profile"
    echo "invalid profile name: $profile" 1>&2
    return 1
  end
  if _flk_use_direnv
    set -lx FLK_FLAKE_REF ".#$profile"
    set -lx FLK_PROFILE ".#$profile"
    direnv reload
  else
    set -lx FLK_FLAKE_REF ".#$profile"
    set -lx FLK_PROFILE ".#$profile"
    set -l flk_shell (test -n "$SHELL"; and echo "$SHELL"; or echo "/bin/sh")
    _flk_exec_nix_develop ".#$profile" "$profile" "$flk_shell"
  end
end
"#
    );
}
