# SDKWork Prompts — Database Design

Status: active  
Owner: prompts-platform  
Updated: 2026-06-29

## Authoritative contract

`specs/prompts-ai-database.schema.yaml`  
Baseline DDL: `database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql`

## Design model

Industry-aligned **definition → version → binding** pattern:

```
ai_prompt_category
        │
        ▼
   ai_prompt ──────► ai_prompt_version
        │                    │
        └──── ai_prompt_binding (owner: agent / workflow / scene)
        
ai_prompt_template  (marketplace / app catalog, keyed by template_key)

ai_prompt_usage     (append-only audit: render, resolve, catalog)
```

## Tables

| Table | Role |
| --- | --- |
| `ai_prompt_category` | Tenant-scoped navigation and filtering |
| `ai_prompt` | Governed definition (`prompt_key`, lifecycle pointers) |
| `ai_prompt_version` | Immutable body, variable/output schema, model constraints |
| `ai_prompt_binding` | Attach version to runtime owner with policy + snapshot |
| `ai_prompt_template` | Lightweight reusable templates (`template_key`) |
| `ai_prompt_usage` | Audit trail without storing full prompt bodies |

## Naming rules

- Module prefix `ai_` per `DATABASE_SPEC.md`
- No redundant suffixes (`_event`, `_json` on column names where JSON type is explicit)
- `template_key` distinguishes marketplace string keys from `prompt_id` bigint FKs

## Multi-tenancy and security

All tenant tables carry `tenant_id`, `organization_id`, `data_scope`, optimistic `version`, soft delete (`deleted_at`, `deleted_by`), and `metadata` JSON per L2 compliance.

Queries MUST filter by `tenant_id` and `organization_id`. Cross-tenant reads are forbidden at the repository layer.

## Indexes (core paths)

- List prompts by category/type: `idx_ai_prompt_category`, `idx_ai_prompt_type`
- List versions: `idx_ai_prompt_version_prompt`
- Resolve bindings: `idx_ai_prompt_binding_owner`, `idx_ai_prompt_binding_prompt`
- Usage audit: `idx_ai_prompt_usage_tenant_created`, `idx_ai_prompt_usage_prompt`

## Ownership boundaries

| Concern | Owner |
| --- | --- |
| Users, tenants, sessions | IAM / appbase |
| Prompt definitions and versions | sdkwork-prompts |
| Agent runtime orchestration | sdkwork-kernel (via `sdkwork-intelligence-prompts-ai-contract`) |

## Verification

```bash
pnpm run db:validate
node tests/schema/prompts-ai-schema.test.mjs
```
