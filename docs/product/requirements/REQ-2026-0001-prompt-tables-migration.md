# REQ-2026-0001: 提示词表迁移至 sdkwork-prompts（ai_ 前缀）

```yaml
id: REQ-2026-0001
title: Consolidate prompt tables into sdkwork-prompts with ai_ prefix
owner: prompts-platform
status: completed
source: platform
problem: >
  Prompt persistence was duplicated across services with inconsistent prefixes.
goals:
  - All prompt-owned tables live in sdkwork-prompts with ai_ prefix
  - sdkwork-kernel depends on sdkwork-intelligence-prompts-ai-contract only
  - No forum (prm_) tables in this application
acceptance_criteria:
  - specs/prompts-ai-database.schema.yaml lists six ai_ tables
  - database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql creates all six tables
  - pnpm verify passes including no-forum-debt.test.mjs
  - crates/sdkwork-intelligence-prompts-ai-contract exists in workspace
```

## Table mapping (final)

| Legacy | Target |
| --- | --- |
| claw-router `ai_prompt*` | `ai_prompt`, `ai_prompt_version`, `ai_prompt_binding` |
| kernel `a_agent_prompt_template` | `ai_prompt_template` |
| `prm_category` | `ai_prompt_category` |
| `prm_usage_event` | `ai_prompt_usage` |
| `prm_template*` | removed — use `ai_prompt` + `ai_prompt_version` |
| `prm_*` forum | removed from sdkwork-prompts |
