# SQL Deployments (Legacy)

This directory is **deprecated**. Prompts database lifecycle assets are owned by the canonical `database/` module at the application root.

## Authoritative Sources

| Asset | Path |
| --- | --- |
| Schema contract | `database/contract/schema.yaml` (synced from `specs/forum-database.schema.yaml`) |
| Table registry | `database/contract/table-registry.json` |
| Baseline DDL | `database/ddl/baseline/postgres/0001_prm_baseline.sql` |
| Migrations | `database/migrations/postgres/` |
| Seeds | `database/seeds/` |
| Drift policy | `database/drift/policy.yaml` |

## Commands

From the forum application root:

```bash
pnpm db:validate
pnpm db:init
pnpm db:migrate
pnpm db:drift:check
```

## Legacy File

`forum-ddl-postgresql.sql` is retained only as a historical snapshot. Do not edit it for new schema changes; update `specs/forum-database.schema.yaml` and sync into `database/contract/schema.yaml` instead.
