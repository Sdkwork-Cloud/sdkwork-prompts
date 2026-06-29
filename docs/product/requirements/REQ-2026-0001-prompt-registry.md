# REQ-2026-0001: Prompt registry

Status: active  
Owner: prompts-platform  
Updated: 2026-06-29

## Scope

`sdkwork-prompts` owns intelligence-domain prompt persistence and HTTP APIs only.

## Functional requirements

1. Six `ai_` tables per `specs/prompts-ai-database.schema.yaml`
2. Three API surfaces (app, backend, open) with OpenAPI authority under `apis/`
3. Generated SDK families under `sdks/` with composed facades
4. Rust gateway `sdkwork-prompts-standalone-gateway` implementing all routes

## Non-functional requirements

1. `pnpm verify` passes (contract, SDK, schema, database framework)
2. Response envelope alignment per API_SPEC section 15
3. Database lifecycle via `sdkwork-database` CLI (`pnpm db:*`)

## Acceptance

```bash
pnpm verify
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
```
