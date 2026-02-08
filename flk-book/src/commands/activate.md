# flk activate

Enter the development environment defined by your flake.

```bash
flk activate
```

**Notes**
- Runs `nix develop --impure` (current profile support planned).
- Add the shell hook for automatic refresh/switching:

```bash
eval "$(flk hook bash)"   # or zsh/fish
```
