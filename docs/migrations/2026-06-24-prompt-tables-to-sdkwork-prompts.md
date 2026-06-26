# Migration: Prompt tables to sdkwork-prompts

Status: completed (2026-06-26)

## Target schema

Authoritative: `specs/prompts-ai-database.schema.yaml`

Tables:

- `ai_prompt_category`
- `ai_prompt`
- `ai_prompt_version`
- `ai_prompt_binding`
- `ai_prompt_template`
- `ai_prompt_usage`

## Bootstrap

```bash
pnpm db:bootstrap
```

## Verification

```bash
pnpm verify
```
