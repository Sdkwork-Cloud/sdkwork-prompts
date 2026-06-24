# REQ-2026-0001: 提示词表迁移至 sdkwork-prompts（ai_ 前缀）

```yaml
id: REQ-2026-0001
title: Consolidate prompt tables into sdkwork-prompts with ai_ prefix
owner: prompts-platform
status: in-progress
source: platform
problem: >
  Prompt persistence and CRUD are duplicated across sdkwork-claw-router,
  sdkwork-kernel, and sdkwork-prompts with inconsistent table prefixes
  (ai_, a_, prm_) violating DATABASE_SPEC.md.
goals:
  - All prompt-owned tables live in sdkwork-prompts with ai_ prefix
  - sdkwork-kernel depends on sdkwork-intelligence-prompts-ai-contract only for prompt ports
  - sdkwork-claw-router and sdkwork-kernel remove local prompt table DDL and CRUD
non_goals:
  - Renaming forum tables (prm_topic, prm_topic_reply, etc.)
  - Moving agent runtime prompt_optimizations endpoint
  - Production cutover in this REQ alone (covered by migration runbook)
users:
  - platform operators
  - app developers
  - sdkwork-kernel agent-business module
  - sdkwork-claw-router routing layer
acceptance_criteria:
  - specs/prompts-ai-database.schema.yaml lists ai_prompt, ai_prompt_version, ai_prompt_binding, ai_agent_prompt_template, ai_prompt_category, ai_prompt_usage_event
  - database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql creates all six tables
  - database/contract/schema.yaml uses ai_ table names only (no prm_template*)
  - docs/product/prd/PRD.md sections 1-8 are populated
  - apis/*/intelligence/prompts/openapi.yaml x-sdkwork-resource values use ai_prompt or ai_prompt_version
  - tests/schema/prompts-ai-schema.test.mjs passes in pnpm verify
  - crates/sdkwork-intelligence-prompts-ai-contract exists and is listed in workspace Cargo.toml
  - sdkwork-agent-business/Cargo.toml declares path dependency on sdkwork-intelligence-prompts-ai-contract (kernel PR may follow P0)
non_functional_requirements:
  security: Tenant isolation via tenant_id, organization_id, data_scope on all ai_ tenant tables per DATABASE_SPEC.md
  privacy: Prompt content may contain PII; audit via ai_prompt_usage_event without logging full body in application logs
  performance: List queries use indexes idx_ai_prompt_category, idx_ai_prompt_type, idx_ai_prompt_version_prompt
  reliability: Soft delete via deleted_at; version immutability after publish
affected_surfaces:
  - api
  - sdk
  - backend
  - pc
trace:
  specs:
    - DATABASE_SPEC.md
    - API_SPEC.md
    - DOMAIN_SPEC.md
    - REQUIREMENTS_SPEC.md
  components:
    - specs/prompts-ai-database.schema.yaml
    - database/
    - crates/sdkwork-intelligence-prompts-ai-contract
    - ../sdkwork-kernel/sdkwork-agent-business
    - ../sdkwork-claw-router/services/sdkwork-clawrouter-router-service
verification:
  - pnpm verify
  - cargo test -p sdkwork-intelligence-prompts-ai-contract
```

## Implementation Notes

### Source mapping

| Legacy | Target |
| --- | --- |
| claw-router `ai_prompt` | `ai_prompt` (same columns, owner → sdkwork-prompts) |
| claw-router `ai_prompt_version` | `ai_prompt_version` |
| claw-router `ai_prompt_binding` | `ai_prompt_binding` |
| kernel `a_agent_prompt_template` | `ai_agent_prompt_template` |
| prompts contract `prm_category` | `ai_prompt_category` |
| prompts contract `prm_usage_event` | `ai_prompt_usage_event` |
| prompts contract `prm_template*` | **removed** — superseded by ai_prompt + ai_prompt_version |

### Kernel dependency

```toml
# sdkwork-agent-business/Cargo.toml (target)
sdkwork-intelligence-prompts-ai-contract = { path = "../../sdkwork-prompts/crates/sdkwork-intelligence-prompts-ai-contract" }
```

Agent business MUST NOT import `sdkwork-intelligence-prompts-repository-sqlx` directly.

### Claw-router removal checklist

- [ ] Remove `ai_prompt*` from `database/contract/schema.yaml` and generated schema
- [ ] Delete `admin_prompt_store.rs` (postgres/sqlite)
- [ ] Delete or proxy `admin_prompts.rs` to sdkwork-prompts backend SDK
- [ ] Update route manifests to forward `/backend/.../prompts` to prompts service
- [ ] Remove PC `promptService.ts` local SQL paths
