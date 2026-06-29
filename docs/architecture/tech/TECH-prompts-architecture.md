# SDKWork Prompts — Architecture

Status: active  
Owner: prompts-platform  
Updated: 2026-06-26

## Bounded context

`sdkwork-prompts` owns **prompt persistence and governance** for the `intelligence` domain.

### In scope

- Prompt definitions, versions, publish/render lifecycle
- Bindings to agents, workflows, and scenes
- Marketplace-style `ai_prompt_template` catalog
- Public read-only catalog (Open API)
- Usage audit (`ai_prompt_usage`)

### Out of scope

- Forum, community, moderation, or messaging
- LLM inference execution (kernel / router responsibilities)
- User/tenant IAM source tables

## Module layout

```
apis/                          # Authored OpenAPI (app, backend, open)
specs/prompts-ai-database.schema.yaml
database/                      # DDL, contract, seeds
crates/
  sdkwork-intelligence-prompts-ai-contract/     # Ports + DTOs (kernel consumer)
  sdkwork-intelligence-prompts-ai-repository-sqlx/
  sdkwork-prompts-database-host/
  sdkwork-prompts-service-host/
  sdkwork-prompts-standalone-gateway/
sdks/                          # Materialized OpenAPI + composed facades
```

## API surfaces

| Surface | Prefix | Auth | Purpose |
| --- | --- | --- | --- |
| app-api | `/app/v3/api` | Dual-token | App templates + agent template catalog |
| backend-api | `/backend/v3/api` | Dual-token | Admin definitions, versions, bindings |
| open-api | `/prompts/v3/api` | Public reads | Published catalog |

## Integration

- **Kernel** depends on `sdkwork-intelligence-prompts-ai-contract` only — never on SQLx repository or DDL.
- **IAM** optional via `SDKWORK_PROMPTS_IAM_ENABLED`; resolves tenant/org/user into request context.

## Principles

- Contract-first: OpenAPI and database schema precede implementation
- High cohesion: all `ai_` DDL owned here
- Low coupling: ports for kernel; no raw HTTP in consumers
- Open/closed: new prompt types and binding roles extend via schema and API without cross-domain table ownership

## Verification

```bash
cargo build --workspace
pnpm verify
```
