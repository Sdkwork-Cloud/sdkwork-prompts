# sdkwork-prompts

SDKWork Prompts — intelligence-domain prompt template management (templates, versions, variables) with Rust HTTP backend and PC browser application.

## Standards

- Repository instructions: `AGENTS.md`
- Local component specs: `specs/README.md`
- Root SDKWork standards: `../sdkwork-specs/README.md`

## Capabilities

| Surface | Path | Purpose |
| --- | --- | --- |
| App API | `apis/app-api/intelligence/prompts/` | Authenticated template CRUD for apps |
| Backend API | `apis/backend-api/intelligence/prompts/` | Operator governance |
| Open API | `apis/open-api/intelligence/prompts/` | Public catalog reads |
| PC app | `apps/sdkwork-prompts-pc/` | Browser/desktop prompt workspace UI |

## Platform integration

- `sdkwork-web-framework` — HTTP route crates and API server
- `sdkwork-database` — `database/` lifecycle and SQLx repositories
- `sdkwork-utils` — shared Rust/TypeScript helpers (`sdkwork-utils-rust`, `@sdkwork/utils`)
- `sdkwork-discovery` — deferred until RPC services exist

## Verification

```bash
pnpm api:materialize
pnpm check
pnpm verify
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Application Roots

- [apps directory index](apps/README.md)
