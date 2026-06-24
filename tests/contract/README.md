# Contract Tests

Validates forum contract files against SDKWork standards.

## Test File

- `prompts-contract.test.mjs` - Validates:
  - Required contract files exist
  - No forbidden "thread" term in contract files
  - Open API declares anonymous security
  - Route manifests have correct surface, domain, capability, prefix
  - All routes have valid operationId format
  - Auth mode matches surface (public for open-api, dual-token for app/backend)
  - Assembly metadata matches expected values

## Run

```bash
node tests/contract/prompts-contract.test.mjs
```
