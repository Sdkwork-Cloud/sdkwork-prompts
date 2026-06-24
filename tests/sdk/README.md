# SDK Tests

Validates SDK generation inputs and composed facades.

## Test File

- `forum-sdk.test.mjs` - Validates:
  - sdkgen configs have correct apiAuthority, sdkFamily, prefix, surface
  - Route manifest entries have required fields (operationId, method, path, surface, authMode)
  - Assembly metadata is complete
  - Composed facades are implemented (no TODO stubs)

## Run

```bash
node tests/sdk/forum-sdk.test.mjs
```
