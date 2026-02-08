# Development Setup

## Requirements
- Nix with flakes enabled (Lix or Determinate installer recommended)
- Rust toolchain (if building from source)

## Building from source
```bash
git clone https://github.com/AEduardo-dev/flk.git
cd flk
cargo build --release
```

## Running the CLI locally
- Use `cargo run -- <command>` during development.
- For shell integration while iterating, add: `eval "$(flk hook bash)"` (or zsh/fish) to your shell.

## Testing docs locally
- Book: `cd flk-book && mdbook serve` (if mdBook is installed).
- API docs: `cargo doc --all-features --open`.
