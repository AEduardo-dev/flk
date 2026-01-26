# Installation

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
  - We recommend the Lix package manager for easy Nix installation: [Lix](https://lix.systems/install/) since it comes with flakes enabled by default.
  - Or using the Determinate System installer: [Determinate](https://determinate.systems/nix-installer/), as it provides a user-friendly way to install (and uninstall) Nix.
- Rust 1.83+ (if building from source)

### Hooks

If you would like to use hot reloading and switching features, you will need to add the following shell hook to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.):

Example for **bash**:

```sh
# flk shell hook
if command -v flk &> /dev/null; then
  eval "$(flk hook bash)"
fi
```

Support for other shells (zsh and fish) is also available via `flk hook <shell>`.

### From Source

```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
cargo build --release
sudo cp target/release/flk /usr/local/bin/
```

### From Cargo (crates.io)

```bash
cargo install flk
```

### From Release Binaries

1. Go to https://github.com/AEduardo-dev/flk/releases
2. Download the archive for your OS/arch (Linux x86_64, macOS Intel, macOS ARM).
3. Unpack and place `flk` in your PATH.

### Nix (with Cachix binaries)

This flake is prebuilt and published to Cachix.

1. Install Cachix (once):

```sh
nix profile install nixpkgs#cachix
```

2. Trust the cache:

```sh
cachix use flk-cache
```

or add the following substituters and trusted-public-keys to your `nix.conf` content:

```
substituters = https://flk-cache.cachix.org  ...
trusted-public-keys = flk-cache.cachix.org-1:6xobbpP9iIK5sIH/75DQrsJYKN/61nTOChcH9MJnBR0=  ...
```

3. Use the flake:

- Run (no install): `nix run github:AEduardo-dev/flk#flk`
- Install to your profile: `nix profile install github:AEduardo-dev/flk#flk`

### Nix – Using as a flake input

You can consume `flk` from another flake either directly or via the overlay.

**Direct (no overlay):**

```nix
{
  description = "My NixOS config with flk";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flk.url = "github:AEduardo-dev/flk";
  };

  outputs = { self, nixpkgs, flk, ... }:
    let
      system = "x86_64-linux"; # set per host
      pkgs = import nixpkgs { inherit system; };
    in {
      nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          {
            environment.systemPackages = [
              flk.packages.${system}.flk
            ];
          }
        ];
      };
    };
}
```

**With overlay (exposes `pkgs.flk`):**

```nix
{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flk.url = "github:AEduardo-dev/flk";

  outputs = { self, nixpkgs, flk, ... }:
    let
      system = "x86_64-linux"; # set per host

      pkgs = import nixpkgs {
        inherit system;
        overlays = [ flk.overlay ];
      };
    in {
      nixosConfigurations.myhost = nixpkgs.lib.nixosSystem {
        inherit system;
        modules = [
          { environment.systemPackages = [ pkgs.flk ]; }
        ];
      };
    };
}
```

**Home Manager example (per-user install via flake):**

```nix
{
  inputs.flk.url = "github:AEduardo-dev/flk";

  outputs = { self, flk, ... }: {
    homeConfigurations.myhost = {
      # ...
      home.packages = [ flk.packages.${system}.flk ];
    };
  };
}
```

### Architectures covered by the cache

- x86_64-linux
- x86_64-darwin
- aarch64-darwin
- aarch64-linux (built via qemu on CI; may be slower/occasional misses)

Other architectures will fall back to building from source.

## Next Steps

- [Getting Started](./getting-started.md)
- [Core Concepts](./concepts.md)
