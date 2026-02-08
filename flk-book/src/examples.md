# Examples

## Python data science
```bash
flk init --template python
flk add python312Packages.numpy python312Packages.pandas python312Packages.matplotlib jupyter
flk cmd add notebook "jupyter notebook --port=8888"
flk env add JUPYTER_CONFIG_DIR "./.jupyter"
flk activate
notebook
```

## Rust web backend
```bash
flk init --template rust
flk add postgresql redis
flk cmd add dev "cargo watch -x run"
flk cmd add migrate "sqlx migrate run"
flk env add DATABASE_URL "postgresql://localhost/myapp"
flk activate
dev
```

## Node full-stack
```bash
flk init --template node
flk add postgresql docker-compose
flk cmd add dev "npm run dev"
flk cmd add db "docker-compose up -d postgres"
flk env add NODE_ENV "development"
flk activate
db
dev
```
