# Prompt AI Tables Migration

Status: draft
Updated: 2026-06-24
Related: REQ-2026-0001, PRD.md

## Overview

Move prompt fact tables from `sdkwork-claw-router` and `sdkwork-kernel` into `sdkwork-prompts`. All prompt tables use the `ai_` module prefix per `DATABASE_SPEC.md`.

> **Note:** Repository name in user request was `sdkwork-craw-router`; the actual source is `sdkwork-claw-router`.

## Pre-migration inventory

### sdkwork-claw-router (remove after cutover)

| Object | Path |
| --- | --- |
| `ai_prompt` DDL | `generated/schema/postgres/schema.sql` |
| `ai_prompt_version` DDL | same |
| `ai_prompt_binding` DDL | same |
| Postgres store | `services/.../postgres/admin_prompt_store.rs` |
| SQLite store | `services/.../sqlite/admin_prompt_store.rs` |
| HTTP API | `services/.../api/admin_prompts.rs` |

### sdkwork-kernel (remove after cutover)

| Object | Path |
| --- | --- |
| `a_agent_prompt_template` DDL | `sdkwork-agent-business/specs/sql/agent_business_postgres.sql` |
| Domain + persistence | `sdkwork-agent-business/src/domain.rs`, `persistence.rs`, `application.rs` |

## Target schema (sdkwork-prompts)

Authoritative: `specs/prompts-ai-database.schema.yaml`  
Baseline: `database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql`

## Migration paths

### A. Same PostgreSQL database (dev / single-tenant)

1. Apply `0001_prompts_ai_baseline.sql` if tables do not exist.
2. Copy data:

```sql
INSERT INTO ai_prompt SELECT * FROM ai_prompt; -- no-op if renamed in place
-- If kernel table exists:
INSERT INTO ai_agent_prompt_template (...)
SELECT id, uuid, tenant_id, organization_id, owner_user_id, prompt_id, code, ...
FROM a_agent_prompt_template;
```

3. Drop legacy tables only after application cutover:

```sql
DROP TABLE IF EXISTS a_agent_prompt_template;
-- claw-router: drop only when router no longer references them
```

### B. Cross-database (recommended production)

1. Deploy `sdkwork-prompts-api-server` with prompts database.
2. Export from claw-router DB: `ai_prompt`, `ai_prompt_version`, `ai_prompt_binding`.
3. Import into prompts DB preserving `id` and `uuid` for binding stability.
4. Point claw-router admin routes to prompts backend SDK (no local store).
5. Deploy kernel with contract client; disable local `a_agent_prompt_template` writes.

### C. Rename in place (maintenance window)

```sql
ALTER TABLE ai_prompt RENAME TO ai_prompt_legacy;
-- apply new baseline if column drift
INSERT INTO ai_prompt SELECT ... FROM ai_prompt_legacy;
DROP TABLE ai_prompt_legacy;
```

## Rollback

- Keep legacy tables for one release with read-only flag.
- Router/kernel feature flag `SDKWORK_PROMPTS_LOCAL_PROMPT_STORE=true` re-enables old path until rollback window closes.

## Verification

```bash
cd sdkwork-prompts
pnpm verify
psql "$DATABASE_URL" -c "\dt ai_*"
```

Expected tables: `ai_prompt_category`, `ai_prompt`, `ai_prompt_version`, `ai_prompt_binding`, `ai_agent_prompt_template`, `ai_prompt_usage_event`.
