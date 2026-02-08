# Custom Commands

Custom commands let you define reusable scripts in your flk environment.

```bash
flk cmd add dev "npm run dev"
flk cmd add test "cargo test --all"
flk cmd list
flk cmd remove dev
```

Commands are stored in your profile and become available when the environment is activated.
