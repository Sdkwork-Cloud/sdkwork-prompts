# sdkwork-prompts

SDKWork Prompts — intelligence-domain prompt registry (definitions, versions, bindings, templates, catalog) with Rust HTTP backend and PC browser application.

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
| PC app | `apps/sdkwork-prompts-pc/` | Browser prompt workspace UI |

## Platform integration

| Framework | Integration |
| --- | --- |
| `sdkwork-web-framework` | Gateway bootstrap, infra routes, `ProblemDetail` / `SdkWorkApiResponse` |
| `sdkwork-database` | `database/` lifecycle, SQLx repositories, ops HTTP |
| `sdkwork-utils` | `sdkwork-utils-rust` (Rust envelope), `@sdkwork/utils` (TypeScript HTTP helpers) |
| `sdkwork-iam` | Optional dual-token session resolution |
| `sdkwork-drive` | Declared SDK dependency; used when binary attachment features ship |
| `sdkwork-discovery` | Not used (HTTP-only; add with RPC services) |

See [TECH-prompts-integration-roadmap.md](docs/architecture/tech/TECH-prompts-integration-roadmap.md) for integration details.

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
