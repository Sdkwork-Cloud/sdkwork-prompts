# SDK Tests

SDK generation and composed facade tests belong here.

## Test Files

- `tests/sdk/forum-sdk.test.mjs` - Validates:
  - sdkgen configs have correct apiAuthority, sdkFamily, prefix, surface
  - Route manifest entries have required fields
  - Assembly metadata is complete
  - Composed facades are implemented (no TODO stubs)

## Planned Tests

sdkgen inspect tests will be added after generated outputs are produced:
- Generated package.json validation
- Generated TypeScript type completeness
- Generated Rust module exports
