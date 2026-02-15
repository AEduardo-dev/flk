# Examples

This page provides complete, practical examples for common development scenarios.

## Python Data Science Environment

Set up a complete data science environment with Jupyter, scientific packages, and helper commands.

```bash
# Initialize Python project
flk init --template python

# Add Python and data science packages
flk add python312
flk add python312Packages.numpy
flk add python312Packages.pandas
flk add python312Packages.matplotlib
flk add python312Packages.scikit-learn
flk add jupyter

# Add convenience commands
flk cmd add notebook "jupyter notebook --port=8888"
flk cmd add lab "jupyter lab --port=8888"
flk cmd add ipython "python -m IPython"

# Configure environment
flk env add JUPYTER_CONFIG_DIR "./.jupyter"
flk env add PYTHONDONTWRITEBYTECODE "1"

# Enter environment and start working
flk activate
notebook
```

## Rust Web Backend

Full-stack Rust development with database, caching, and development tools.

```bash
# Initialize Rust project
flk init --template rust

# Add development dependencies
flk add postgresql
flk add redis
flk add sqlx-cli
flk add cargo-watch

# Add development commands
flk cmd add dev "cargo watch -x run"
flk cmd add test "cargo test --all-features"
flk cmd add migrate "sqlx migrate run"
flk cmd add db "psql $DATABASE_URL"

# Configure database connection
flk env add DATABASE_URL "postgresql://localhost/myapp"
flk env add REDIS_URL "redis://localhost:6379"
flk env add RUST_LOG "debug"

# Activate and start development
flk activate
dev
```

## Node.js Full-Stack Application

Modern JavaScript/TypeScript development with database and tooling.

```bash
# Initialize Node project
flk init --template node

# Add runtime and tools
flk add nodejs_20
flk add postgresql
flk add docker-compose
flk add nodePackages.typescript
flk add nodePackages.eslint

# Add development commands
flk cmd add dev "npm run dev"
flk cmd add build "npm run build"
flk cmd add db:start "docker-compose up -d postgres"
flk cmd add db:stop "docker-compose down"
flk cmd add lint "npm run lint"
flk cmd add typecheck "tsc --noEmit"

# Set environment variables
flk env add NODE_ENV "development"
flk env add DATABASE_URL "postgresql://localhost/myapp"
flk env add PORT "3000"

# Start development
flk activate
db:start
dev
```

## Go Microservice

Go development with common tools and testing setup.

```bash
# Initialize Go project
flk init --template go

# Add Go and tools
flk add go
flk add gopls
flk add golangci-lint
flk add mockgen
flk add protobuf

# Add commands
flk cmd add run "go run ./cmd/server"
flk cmd add test "go test ./..."
flk cmd add lint "golangci-lint run"
flk cmd add proto "protoc --go_out=. --go-grpc_out=. ./proto/*.proto"
flk cmd add build "go build -o bin/server ./cmd/server"

# Configure
flk env add GOPROXY "https://proxy.golang.org,direct"
flk env add CGO_ENABLED "0"

# Activate
flk activate
run
```

## Multi-Language Monorepo

For projects with multiple languages, you can create separate profiles.

```bash
# Initialize with generic template
flk init --template generic

# The default profile is in .flk/profiles/default.nix
# You can create additional profiles by copying and modifying:
# .flk/profiles/frontend.nix
# .flk/profiles/backend.nix

# Add shared tools to default profile
flk add git
flk add docker-compose
flk add make

# Add commands that work across the project
flk cmd add up "docker-compose up -d"
flk cmd add down "docker-compose down"
flk cmd add logs "docker-compose logs -f"
```

## Pinning Package Versions

When you need reproducible builds with specific versions:

```bash
# Pin specific package versions
flk add ripgrep --version 14.1.0
flk add nodejs --version 20.10.0
flk add python3 --version 3.11.6

# View what's pinned
flk list packages

# The version info is stored in .flk/pins.nix
```

## Using with Direnv

Automatically activate environments when entering directories:

```bash
# Set up direnv integration
flk direnv init

# Allow the .envrc file
direnv allow

# Now the environment loads automatically when you cd into the project
cd ~/projects/myapp  # Environment activates
cd ~                 # Environment deactivates
```

## Container Export for CI/CD

Export your development environment for use in CI or on machines without Nix:

```bash
# Export as Docker image
flk export --format docker

# Export as Podman image
flk export --format podman

# Export configuration as JSON (useful for debugging or other tools)
flk export --format json
```
