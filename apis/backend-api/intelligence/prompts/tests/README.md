# Backend API Contract Tests

Contract tests for backend-api forum operations.

## Test Coverage

- OpenAPI parity: Routes must match `apis/backend-api/prompts/openapi.yaml`
- Route manifest parity: Routes must match `sdks/_route-manifests/backend-api/*.route-manifest.json`
- Auth mode: All operations must use dual-token security
- Prefix: All paths must start with `/backend/v3/api`

## Test Files

- `tests/static/prompts-contract-boundary.test.mjs` - Boundary checks
- `tests/contract/prompts-contract.test.mjs` - Contract validation
