# ilbal — CLI

Manage ilbal.dev projects locally: init, run containers, ingest data, and prepare for cloud deployment.

## User Guide

### Prerequisites

- Rust toolchain
- Docker **or** Podman
- (Optional) [kompose](https://kompose.io/) + [helm](https://helm.sh/) for Kubernetes export

### Installation

```bash
cargo install --path ilbal
```

### Commands

| Command | Status | Description |
|---|---|---|
| `ilbal init` | ✅ | Creates a new project interactively. Prompts for name, runtime (Docker/Podman), credentials. Writes `<name>/ilbal/config.toml`. |
| `ilbal start` | ✅ | Starts the local dev stack (postgresql, pgadmin4, pgdog) |
| `ilbal stop` | ✅ | Stops containers |
| `ilbal status` | ✅ | Shows container health |
| `ilbal ingest` | ✅ | Runs data ingestion via Docker |
| `ilbal pull` | 🚧 | Pulls required images |
| `ilbal pgbranch` | 🚧 | Database branching |
| `ilbal pgroll` | 🚧 | Database migrations |

### Quick start

```bash
ilbal init
cd <project>
ilbal start
```

## Architecture

### Where ilbal-cli fits in the ilbal.dev workspace

```
ilbal.dev/
├── ilbal-cli/             ← This repo. Orchestrates the local dev experience.
├── ilbal-oci-images/      Docker images: ilbal-postgresql, ingest, (future: pgbranch, pgroll)
├── ilbal-devops/          Docker/Podman testing and infrastructure
├── ilbal-pgrx-extensions/ PostgreSQL extensions (pgrx)
└── garage/                S3-compatible bucket
```

### Local dev stack

`ilbal start` spins up three containers:

| Container | Role |
|---|---|
| **ilbal-postgresql** | Custom PostgreSQL image built from `ilbal-oci-images` |
| **pgAdmin4** | Web UI for database management |
| **pgDog** | Connection pooler / proxy |

### Development flow

1. **`ilbal init`** — scaffolds a project, stores config in `<name>/ilbal/config.toml`, writes rendered compose and Kubernetes files
2. **`ilbal start`** — launches containers via Docker Compose or Podman
3. **Develop your backend** — use the running PostgreSQL, pgAdmin4, and supporting tools
4. **Future: cloud deploy** — the Helm chart in `<name>/ilbal/kube/helm/` is the path to production

The CLI itself stays thin — it delegates to `docker compose` / `podman play kube` for container management, and to `ilbal-oci-images` for the actual database and tooling images.
