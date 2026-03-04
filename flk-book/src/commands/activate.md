# flk activate

Enter the Nix development shell for your project.

```bash
flk activate
flk activate --profile backend
```

**Options**
- `-p, --profile <PROFILE>`: Activate a specific profile instead of the default

**Behavior**
- Runs `nix develop .#<profile> --impure` to enter the dev shell
- Uses GC root pinning (`.flk/.nix-profile-<profile>`) for faster re-activation on subsequent runs
- Uses standard [profile resolution](../concepts.md#profiles) when `--profile` is not specified
- Custom commands and environment variables from the profile are available inside the shell

**Notes**
- For automatic environment switching when navigating between projects, add the shell hook:
  ```bash
  eval "$(flk hook bash)"   # or zsh/fish
  ```
- For automatic activation when entering a directory, use [direnv integration](./direnv.md)

**See Also**
- [flk hook](./hook.md) — for `refresh` and `switch` commands
- [flk direnv](./direnv.md) — for automatic directory-based activation
