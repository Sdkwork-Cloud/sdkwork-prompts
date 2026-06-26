# Prompts Specs

Authoritative contracts for the `intelligence/prompts` capability.

## Contracts

| File | Purpose |
| --- | --- |
| `component.spec.json` | SDKWork component contract for this application root |
| `prompts-ai-database.schema.yaml` | Database schema registry (6 `ai_` tables) |
| `topology.spec.json` | Deployment topology and process orchestration |

## Database tables (`ai_`)

| Group | Tables |
| --- | --- |
| taxonomy | `ai_prompt_category` |
| core | `ai_prompt`, `ai_prompt_version` |
| binding | `ai_prompt_binding` |
| template | `ai_prompt_template` |
| audit | `ai_prompt_usage` |

## Verification

```bash
pnpm run db:validate
node tests/schema/prompts-ai-schema.test.mjs
```
