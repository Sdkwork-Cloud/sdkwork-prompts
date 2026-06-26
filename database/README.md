# PROMPTS Database Module

Canonical lifecycle assets for `sdkwork-prompts` per `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `prompts`
- serviceCode: `PROMPTS`
- tablePrefix: `ai_`

## Commands

```bash
pnpm run db:validate
pnpm run db:materialize:contract
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
```

## Contract sources

- Semantic schema registry: `specs/prompts-ai-database.schema.yaml`
- Framework contract: `database/contract/*`
- Baseline DDL: `database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql`

## Runtime

PostgreSQL services bootstrap through `sdkwork-prompts-database-host`:

- `bootstrap_prompts_database()` / `bootstrap_prompts_database_from_env()`
