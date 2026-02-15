# Troubleshooting

This page covers common issues and their solutions.

## Nix Not Found

**Symptom:** `command not found: nix` or flk commands fail with Nix errors.

**Solution:**
1. Ensure Nix is installed: `curl -L https://nixos.org/nix/install | sh`
2. Verify installation: `nix --version`
3. Enable flakes in `~/.config/nix/nix.conf`:
   ```
   experimental-features = nix-command flakes
   ```
4. Restart your shell or run `source ~/.bashrc`

## Search/Version Lookup Errors

**Symptom:** `flk search` or `flk add --version` fails with network or evaluation errors.

**Solution:**
- These commands use `nix run github:vic/nix-versions` which requires network access
- Check your internet connection
- Try running directly: `nix run github:vic/nix-versions -- -p ripgrep`
- If behind a proxy, ensure Nix proxy settings are configured

## Lock File Missing or Corrupted

**Symptom:** `error: getting status of flake.lock: No such file or directory`

**Solution:**
```bash
# Generate a new lock file
nix flake lock

# Or reinitialize the project
flk init --force
```

## Shell Hook Not Working

**Symptom:** `switch` and `refresh` commands not available after activating.

**Solution:**
1. Ensure the hook is in your shell profile:
   ```bash
   # For bash (~/.bashrc)
   eval "$(flk hook bash)"
   
   # For zsh (~/.zshrc)
   eval "$(flk hook zsh)"
   
   # For fish (~/.config/fish/config.fish)
   flk hook fish | source
   ```
2. Restart your terminal or source your profile
3. Verify with `type switch` - it should show a function definition

## Direnv Not Loading Environment

**Symptom:** Environment doesn't activate when entering project directory.

**Solution:**
1. Initialize direnv integration: `flk direnv init`
2. Allow the `.envrc` file: `direnv allow`
3. Ensure direnv hook is in your shell profile
4. Check `.envrc` exists and contains `use flake`

## Package Not Found

**Symptom:** `flk add <package>` fails with "package not found".

**Solution:**
1. Search for the correct package name: `flk search <term>`
2. For deep search with versions: `flk deep-search <package>`
3. Package names in Nix may differ from common names (e.g., `ripgrep` not `rg`)

## Profile Errors

**Symptom:** "Profile not found" or profile-related errors.

**Solution:**
1. List available profiles: `flk list profiles`
2. Check `.flk/profiles/` directory exists
3. Ensure profile names contain only alphanumeric characters, dashes, and underscores

## Activation Fails

**Symptom:** `flk activate` or `nix develop` fails with evaluation errors.

**Solution:**
1. Check for syntax errors in `.flk/profiles/*.nix` files
2. Validate the flake: `nix flake check`
3. Try updating inputs: `flk update`
4. Check the error message for specific package or syntax issues

## Container Export Issues

**Symptom:** `flk export --format docker` fails.

**Solution:**
1. Ensure Docker/Podman is installed and running
2. Check you have permissions to run container commands
3. For Docker: `docker info` should succeed
4. For Podman: `podman info` should succeed
