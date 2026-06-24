# Prompts SDK Workspace

This directory is the SDKWork SDK generation workspace for the forum capability.

## SDK Families

| Family | Surface | Prefix | Auth | Status |
|--------|---------|--------|------|--------|
| `sdkwork-prompts-app-sdk` | app-api | `/app/v3/api` | dual-token | Composed facade implemented |
| `sdkwork-prompts-backend-sdk` | backend-api | `/backend/v3/api` | dual-token | Composed facade implemented |
| `sdkwork-prompts-sdk` | open-api | `/prompts/v3/api` | anonymous | Composed facade implemented |

## Structure

```
sdks/
  sdkwork-prompts-app-sdk/
    openapi/                    # sdkgen configs
    composed/src/index.ts       # PromptsAppFacade (22 methods)
    generated/server-openapi/   # sdkgen output (awaiting generation)
  sdkwork-prompts-backend-sdk/
    openapi/
    composed/src/index.ts       # PromptsBackendFacade (30+ methods)
    generated/server-openapi/
  sdkwork-prompts-sdk/
    openapi/
    composed/src/index.ts       # PromptsOpenFacade (8 methods)
    generated/server-openapi/
  _route-manifests/             # Route manifest JSON files
  _shared/                      # Shared schema fragments
  test/                         # SDK tests
```

## Generation

Generated transport output under each family `generated/server-openapi` is generator-owned and must not be hand-edited. Handwritten composition belongs in `composed/`.

Run canonical sdkgen after authority OpenAPI review:
```bash
../sdkwork-sdk-generator/bin/sdkgen.js --input sdks/sdkwork-prompts-app-sdk/openapi/sdkwork-prompts-app-api.sdkgen.yaml
```

## Tests

- `tests/sdk/forum-sdk.test.mjs` - Validates sdkgen configs, route manifests, and composed facades
