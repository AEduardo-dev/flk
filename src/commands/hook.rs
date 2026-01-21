use anyhow::Result;
use clap::ValueEnum;

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum HookShell {
    Bash,
    Zsh,
    Fish,
}

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

refresh() {{
  if _flk_use_direnv; then
    direnv reload
  else
    exec nix develop . --impure
  fi
}}

switch() {{
  local profile="$1"
  if [ -z "$profile" ]; then
    printf 'usage: switch <profile>\n' >&2
    return 1
  fi
  if _flk_use_direnv; then
    FLK_PROFILE="$profile" direnv reload
  else
    exec nix develop .#"$profile" --impure
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

function refresh --description "Reload env (direnv if present, else nix develop)"
  if _flk_use_direnv
    direnv reload
  else
    exec nix develop . --impure
  end
end

function switch --description "Switch profile and reload"
  if test (count $argv) -lt 1
    echo "usage: switch <profile>" 1>&2
    return 1
  end
  set profile $argv[1]
  if _flk_use_direnv
    set -lx FLK_PROFILE $profile
    direnv reload
  else
    exec nix develop .#$profile --impure
  end
end
"#
    );
}
