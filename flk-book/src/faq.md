# FAQ

## General

**What is flk?**  
flk is a CLI tool that simplifies working with Nix flake development environments. It provides commands for managing packages, environment variables, and custom commands without manually editing Nix files.

**Does flk work without Nix installed?**  
No, Nix with flakes enabled is required. However, you can export your environment to Docker/Podman containers using `flk export --format docker` for systems without Nix.

**What Nix version do I need?**  
Any recent Nix version (2.4+) with experimental features `nix-command` and `flakes` enabled.

## Environment Management

**How do I enable auto-switching between projects?**  
Add the hook to your shell profile:
```bash
# Bash
eval "$(flk hook bash)"

# Zsh
eval "$(flk hook zsh)"

# Fish
flk hook fish | source
```
Then use `switch` to reload after changes and `refresh` to re-enter the environment.

**How do I pin a package to a specific version?**  
Use the `--version` flag:
```bash
flk add ripgrep --version 14.1.0
```
This stores version information in `.flk/pins.nix` and locks the nixpkgs commit that contains that version.

**Can I have multiple profiles in one project?**  
Yes! Profiles are stored in `.flk/profiles/`. You can create additional profiles and switch between them. The default profile is set in `.flk/default.nix`.

**How do I see what's currently installed?**  
```bash
flk list             # List packages
flk command list     # List custom commands
flk env list         # List environment variables
flk show             # Show full flake configuration
```

## Updates and Lock Files

**Can I preview updates before applying?**  
Yes, use `flk update --show` which shows the diff and then restores the original lockfile.

**How do I restore a previous lockfile?**  
Backups are stored in `.flk/backups/`. Restore with:
```bash
flk lock restore <backup-name>
```

**What happens when I run `flk update`?**  
It creates a backup of your current lockfile, runs `nix flake update`, and shows you what changed.

## Integration

**Can I use flk with direnv?**  
Yes! Initialize with `flk direnv init` to create a `.envrc` file, then run `direnv allow`. Your environment will automatically load when entering the project directory.

**Does flk support VS Code / other editors?**  
flk works with any editor. For VS Code with direnv, install the direnv extension and initialize flk direnv integration. The environment will be available in integrated terminals.

**Can I export my environment to a container?**  
Yes:
```bash
flk export --format docker   # Creates a Docker image
flk export --format podman   # Creates a Podman image
flk export --format json     # Exports config as JSON
```

## Troubleshooting

**Why can't flk find my package?**  
Package names in Nix may differ from common names. Use `flk search <term>` to find the correct name, or `flk deep-search <package>` for detailed info including available versions.

**Why isn't my shell hook working?**  
Ensure you've added the hook to your shell profile and restarted your terminal. The `switch` and `refresh` commands are only available inside an activated flk environment.
