# SDKWork Prompts — Integration Roadmap

Status: active  
Owner: prompts-platform  
Updated: 2026-06-26

## Current integrations

### IAM (implemented)

When `SDKWORK_PROMPTS_IAM_ENABLED=true`, the API server resolves `Authorization` + `Access-Token` via `sdkwork-iam-web-adapter` and populates `PromptsRequestContext` before handlers run.

- `SDKWORK_PROMPTS_IAM_STRICT=true` rejects invalid sessions on app/backend routes
- `SDKWORK_PROMPTS_IAM_DATABASE_URL` optional; defaults to prompts PostgreSQL pool

### Kernel (contract)

`sdkwork-kernel` / `sdkwork-agent-business` MUST consume:

```
sdkwork-intelligence-prompts-ai-contract
```

Never `sdkwork-intelligence-prompts-ai-repository-sqlx` or local `ai_` DDL.

## Deferred integrations

| Integration | Status | Notes |
| --- | --- | --- |
| Usage audit writes | Schema ready | `ai_prompt_usage` DDL live; async compaction job deferred |
| External search index | Not required | Prompts lists use DB indexes |
| Drive attachments | Not in v1 | No binary storage in prompts tables |
| Notification fanout | Not in v1 | No forum subscription model |

## Environment variables

| Key | Purpose |
| --- | --- |
| `SDKWORK_PROMPTS_DATABASE_URL` | PostgreSQL connection |
| `SDKWORK_PROMPTS_IAM_ENABLED` | Enable IAM session resolution |
| `SDKWORK_PROMPTS_IAM_STRICT` | Fail closed on bad sessions |
| `SDKWORK_PROMPTS_DEFAULT_TENANT_ID` | Open catalog default tenant |

## Verification

```bash
node tests/static/prompts-iam-path-alignment.test.mjs
node tests/static/no-clawrouter-runtime-dependency.test.mjs
```
