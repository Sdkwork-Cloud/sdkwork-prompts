# Open API Contract Tests

Contract tests for open-api forum operations.

## Test Coverage

- Anonymous auth enforcement: All operations must declare `security: []`
- No internal headers: Operations must not declare SDKWork dual-token or business context headers
- Route manifest parity: Routes must match `apis/open-api/prompts/openapi.yaml`
- Auth mode: All operations must use `x-sdkwork-auth-mode: anonymous`

## Test Files

- `tests/static/prompts-contract-boundary.test.mjs` - Boundary checks
- `tests/contract/prompts-contract.test.mjs` - Contract validation
