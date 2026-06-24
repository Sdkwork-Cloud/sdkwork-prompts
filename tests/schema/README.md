# Schema Tests

Validates database schema registry against SDKWork standards.

## Test File

- `prompts-schema.test.mjs` - Validates:
  - All 45 required forum tables are defined
  - All tables use `prm_` prefix
  - Required field sets (tenant_entity, integration_log) exist
  - Required tenant fields (tenant_id, organization_id, etc.) exist
  - All 8 table groups are represented
  - Every table has profile, owner, and description

## Run

```bash
node tests/schema/prompts-schema.test.mjs
```
