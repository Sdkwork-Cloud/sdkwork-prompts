# Docker deployment

Docker Compose stack for local and pre-production prompts API validation.

## Prerequisites

Build context is the **SDKWork workspace root** (parent of `sdkwork-prompts`), because Rust path dependencies include `sdkwork-database`, `sdkwork-web-framework`, `sdkwork-utils`, `sdkwork-id`, and `sdkwork-iam`.

## Run

From this directory:

```bash
docker compose up -d --build
```

## Services

| Service | Port | Description |
| --- | --- | --- |
| `prompts-api` | 8080 | `sdkwork-api-prompts-standalone-gateway` |
| `postgres` | 5432 | PostgreSQL 16 (`sdkwork_ai_dev` database) |

## Environment

The API container sets:

- Structured `SDKWORK_DATABASE_*` values come from `docker-compose.yml`.
- `SDKWORK_PROMPTS_APP_ROOT=/app`

Database migrations run on boot via `sdkwork-database` lifecycle (`autoMigrate`).

## Files

- `Dockerfile.server` — multi-stage build from workspace root
- `docker-compose.yml` — API + PostgreSQL
