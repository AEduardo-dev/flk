# Introduction

**flk** is a modern CLI tool for managing Nix flake development environments with the simplicity of tools like Devbox. It provides an intuitive interface for working with Nix flakes without manually editing configuration files.

## Why flk?

- 🎯 **Smart Initialization**: Auto-detects your project type
- 📦 **Easy Package Management**: Add/remove packages with simple commands
- ⚡ **Custom Commands**: Define reusable shell commands
- 🌍 **Environment Management**: Manage environment variables easily
- 🔒 **Lock File Management**: Version control for your dependencies

## Quick Example

```bash
# Initialize a Rust project
flk init

# Add packages
flk add ripgrep fd-find

# Define a custom command
flk cmd add build "cargo build --release"

# Enter the development environment
nix develop

# Use your custom command
build
```

## Upgrading to v0.5.X (switch/refresh changes)

**WARNING (pre v0.5.0 users):** If you are using `flk < v0.5.0` and you run `flk update` / `nix flake update`, your devshell `switch` / `refresh` behavior may break because the `nix-profile-lib` input may update to a newer version with different activation semantics.

If you intend to stay on `flk < v0.5.0`, use one of these options:

1. **Do not update flake inputs.** Avoid running `flk update` or `nix flake update`. If you already did, restore a previous lockfile backup with:

   ```bash
   flk lock restore <BACKUP>
   ```

2. **Pin `nix-profile-lib` to v0.1.0.** In your `flake.nix`:

   ```nix
   inputs = {
     nix-profile-lib.url = "github:AEduardo-dev/nix-profile-lib?ref=v0.1.0";
   };
   ```

   Then update the lock entry:

   ```bash
   nix flake lock --update-input nix-profile-lib
   ```

   (or `nix flake update --update-input nix-profile-lib`)

Once you upgrade to `flk v0.5.0+`, this restriction is lifted.

## Documentation Structure

- **User Guide**: Learn how to use flk effectively
- **Commands**: Detailed reference for all commands
- **Advanced Topics**: Deep dives into specific features
- **[API Documentation](../api/flk/)**: Internal API reference for contributors

## Getting Help

- [GitHub Issues](https://github.com/AEduardo-dev/flk/issues)
- [Contributing Guide](./contributing.md)
