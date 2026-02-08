# flk completions

Generate shell completions.

```bash
flk completions            # print to stdout
flk completions --install  # auto-install for detected shell
flk completions --shell zsh
```

**Options**
- `--install`: install to the detected shell location
- `--shell <SHELL>`: override shell detection (`bash`, `zsh`, `fish`, etc.)
