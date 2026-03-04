# flk export

Export the current flake configuration to Docker, Podman, or JSON.

```bash
flk export --format docker
flk export --format podman
flk export --format json
flk export --format docker --profile backend
```

**Options**
- `-f, --format <FORMAT>`: Export format — `docker`, `podman`, or `json` (required)
- `-p, --profile <PROFILE>`: Target a specific profile instead of the default

**Formats**

### Docker

Builds a Nix-based Docker image from the flake and loads it into the local Docker daemon.

- Requires Docker to be installed and running
- The image is built via `nix build .#docker-<profile>` and loaded with `docker load`
- Output image is stored at `.flk/result` before loading

### Podman

Same as Docker but uses Podman instead.

- Requires Podman to be installed and running
- Built via `nix build .#podman-<profile>` and loaded with `podman load`

### JSON

Serializes the parsed flake configuration to a `flake.json` file in the project root.

- Includes all profiles, packages, environment variables, and inputs
- Useful for debugging, CI pipelines, or integrating with other tools
- Does not require Docker or Podman

**Notes**
- Docker and Podman exports use `--impure` for Nix builds
- Uses standard [profile resolution](../concepts.md#profiles) when `--profile` is not specified

**See Also**
- [Container Exporting concept](../concepts.md#container-exporting)
- [Container Export example](../examples.md#container-export-for-cicd)
