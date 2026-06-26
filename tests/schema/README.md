# Schema Tests

Validates `ai_` table contracts align across:

- `specs/prompts-ai-database.schema.yaml`
- `database/contract/schema.yaml`
- `database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql`

Run: `node tests/schema/prompts-ai-schema.test.mjs`
