# SDKWork Prompts — Platform Integrations

Status: active  
Owner: prompts-platform  
Updated: 2026-06-29

## Production integrations

| Platform | Crate / package | Role |
| --- | --- | --- |
| `sdkwork-web-framework` | `sdkwork-web-bootstrap`, `sdkwork-web-core` | HTTP infra routes, `ProblemDetail` errors, `SdkWorkApiResponse` success envelope |
| `sdkwork-database` | `sdkwork-prompts-database-host`, `sdkwork-database-ops-http` | PostgreSQL lifecycle, migrations, ops HTTP |
| `sdkwork-utils` | `sdkwork-utils-rust`, `@sdkwork/utils` | Shared envelope types, trace headers, frontend HTTP helpers |
| `sdkwork-iam` | `sdkwork-iam-web-adapter` | Optional dual-token session resolution |

## Cross-domain SDK dependencies

Declared in `specs/component.spec.json` and SDK assembly manifests:

- **IAM (required)** — app session and tenant context for app/backend surfaces
- **Drive (optional)** — required when a feature stores binary assets; prompts v1 stores text in `ai_` tables only

File uploads MUST use `sdkwork-drive` SDK or RPC when introduced. Prompts MUST NOT implement local blob storage.

## Kernel contract boundary

`sdkwork-kernel` / `sdkwork-agent-business` MUST consume `sdkwork-intelligence-prompts-ai-contract` only.  
They MUST NOT depend on `sdkwork-intelligence-prompts-ai-repository-sqlx` or own `ai_` DDL.

## Service discovery

`sdkwork-discovery` is not integrated. The application is HTTP-only today. Add discovery when RPC services are introduced.

## Planned capabilities (post-v1)

| Capability | Trigger |
| --- | --- |
| `ai_prompt_usage` async audit writes | Usage telemetry product requirement |
| Drive-backed attachments | Feature adds binary assets to prompts |
| External search index | Catalog scale exceeds DB index suitability |

## Environment variables

| Key | Purpose |
| --- | --- |
| `SDKWORK_PROMPTS_DATABASE_URL` | PostgreSQL connection |
| `SDKWORK_PROMPTS_IAM_ENABLED` | Enable IAM session resolution |
| `SDKWORK_PROMPTS_IAM_STRICT` | Fail closed on bad sessions |
| `SDKWORK_PROMPTS_DEFAULT_TENANT_ID` | Open catalog default tenant |
| `SDKWORK_PROMPTS_ATTACHMENT_DRIVE_SPACE` | Drive space id when attachment features ship |

## Verification

```bash
pnpm verify
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
```
