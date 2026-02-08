# Troubleshooting

Common fixes:

- **Nix not found:** Ensure Nix is installed with flakes enabled; try `nix --version`.
- **Search errors:** `flk search` and `flk add --version` rely on `nix run github:vic/nix-versions`; verify network access.
- **Lockfile missing:** Run `nix flake lock` or `flk init` to create one.
- **Shell hook not working:** Re-source your shell profile or restart the terminal; confirm `eval "$(flk hook <shell>)"` is present.
- **direnv not loading:** Run `flk direnv init` or `attach`, then `direnv allow`.
