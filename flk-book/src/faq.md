# FAQ

**How do I enable auto-switching between projects?**  
Add the hook to your shell profile: `eval "$(flk hook bash)"` (or zsh/fish).

**How do I pin a package version?**  
Use `flk add <pkg> --version <ver>`; flk writes pin info to `.flk/pins.nix`.

**Can I preview updates before applying?**  
Yes: `flk update --show` restores the lockfile after diffing.

**Does flk work without Nix installed?**  
No, Nix (with flakes) is required. Use container export (`flk export --format docker|podman`) for targets without Nix.
