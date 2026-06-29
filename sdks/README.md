# Prompts SDK workspace

SDK generation workspace for the intelligence **prompts** capability.

## SDK families

| Family | Surface | Prefix | Auth |
| --- | --- | --- | --- |
| `sdkwork-prompts-app-sdk` | app-api | `/app/v3/api` | dual-token |
| `sdkwork-prompts-backend-sdk` | backend-api | `/backend/v3/api` | dual-token |
| `sdkwork-prompts-sdk` | open-api | `/prompts/v3/api` | public read |

## Layout

```
sdks/
  sdkwork-prompts-app-sdk/
    openapi/                    # materialized OpenAPI authority
    composed/src/index.ts       # PromptsAppFacade
    generated/server-openapi/   # sdkgen output (do not hand-edit)
  sdkwork-prompts-backend-sdk/
  sdkwork-prompts-sdk/
  _route-manifests/
  _shared/
```

## Materialize and generate

```bash
pnpm api:materialize
pnpm api:sdkgen
```

Generated transport under `generated/server-openapi` is generator-owned. Handwritten facades belong in `composed/`.

## Tests

```bash
node tests/sdk/prompts-sdk.test.mjs
```
