# SQL Deployments

Database lifecycle is owned by the canonical `database/` module.

| Asset | Path |
| --- | --- |
| Schema contract | `database/contract/schema.yaml` |
| Semantic registry | `specs/prompts-ai-database.schema.yaml` |
| Baseline DDL | `database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql` |

```bash
pnpm db:validate
pnpm db:migrate
```
