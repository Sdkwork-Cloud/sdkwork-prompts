# Developer guide

## Prerequisites

- Rust toolchain (edition 2021)
- Node.js 20+ and pnpm 10+
- PostgreSQL 16+
- SDKWork workspace siblings: `sdkwork-database`, `sdkwork-web-framework`, `sdkwork-utils`, `sdkwork-iam`

`pnpm check` ensures `@sdkwork/utils` is built when `dist/` is missing (`tools/ensure-workspace-deps.mjs`).

## Local setup

```bash
pnpm install
cp configs/local/.env.example .env.local
# Set SDKWORK_PROMPTS_DATABASE_URL
pnpm db:bootstrap
cargo run --bin sdkwork-prompts-standalone-gateway
```

PC app:

```bash
pnpm dev
```

## Contract workflow

```bash
pnpm api:materialize    # apis/ -> sdks/*/openapi/
pnpm api:sdkgen         # regenerate TypeScript transports
pnpm verify
```

## Key paths

| Path | Purpose |
| --- | --- |
| `apis/` | OpenAPI authority |
| `crates/sdkwork-prompts-standalone-gateway/` | HTTP server |
| `sdks/` | Generated SDK families + composed facades |
| `specs/prompts-ai-database.schema.yaml` | Database contract |

See [TECH_ARCHITECTURE.md](../architecture/tech/TECH_ARCHITECTURE.md).
