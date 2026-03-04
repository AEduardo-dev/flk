# flk list

List all packages in the current profile.

```bash
flk list
flk list --profile backend
```

**Options**
- `-p, --profile <PROFILE>`: Target a specific profile instead of the default

**Behavior**
- Reads `.flk/profiles/<profile>.nix` and displays all packages in the `packages = [ ... ];` section
- Uses standard [profile resolution](../concepts.md#profiles) when `--profile` is not specified
- Outputs one package per line with a bullet marker
- Shows an error message if no packages are found

**Example Output**

```
• ripgrep
• fd
• git
• nodejs_20
```
