# Project Templates

flk can scaffold common stacks for you.

```bash
flk init --template rust
flk init --template python
flk init --template node
flk init --template go
flk init --template generic
```

Auto-detection chooses a template based on project files (Cargo.toml, package.json, pyproject/requirements, go.mod). Templates set sensible defaults for build tools, language servers, and common utilities.
