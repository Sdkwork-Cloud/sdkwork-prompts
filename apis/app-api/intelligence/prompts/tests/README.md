# App API Contract Tests

Contract tests for app-api forum operations.

## Test Coverage

- OpenAPI parity: Routes must match `apis/app-api/prompts/openapi.yaml`
- Route manifest parity: Routes must match `sdks/_route-manifests/app-api/*.route-manifest.json`
- Auth mode: All operations must use dual-token security
- Prefix: All paths must start with `/app/v3/api`

## Test Files

- `tests/static/prompts-contract-boundary.test.mjs` - Boundary checks
- `tests/contract/prompts-contract.test.mjs` - Contract validation
