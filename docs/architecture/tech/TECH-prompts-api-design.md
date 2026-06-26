# SDKWork Prompts — API Design

Status: active  
Owner: prompts-platform  
Updated: 2026-06-26

## Surfaces

### App API (`/app/v3/api`)

| Method | Path | operationId |
| --- | --- | --- |
| GET/POST | `/prompts/templates` | `prompts.templates.list` / `.create` |
| GET/PATCH | `/prompts/templates/{templateId}` | `prompts.templates.get` / `.update` |
| GET/POST | `/prompts/templates/{templateId}/versions` | `prompts.templateVersions.list` / `.create` |
| GET | `/prompts/agent_templates` | `prompts.agentTemplates.list` |
| GET | `/prompts/agent_templates/{templateId}` | `prompts.agentTemplates.get` |

Resource mapping: `ai_prompt`, `ai_prompt_version`, `ai_prompt_template`.

### Backend API (`/backend/v3/api`)

| Method | Path | operationId |
| --- | --- | --- |
| GET/POST | `/prompts` | `prompts.admin.definitions.list` / `.create` |
| GET/POST | `/prompts/{promptId}/versions` | `prompts.admin.versions.list` / `.create` |
| POST | `/prompts/versions/{versionId}/publish` | `prompts.admin.versions.publish` |
| POST | `/prompts/versions/{versionId}/render` | `prompts.admin.versions.render` |
| GET/POST | `/prompts/{promptId}/bindings` | `prompts.admin.bindings.list` / `.create` |
| PUT | `/prompts/bindings/{bindingId}` | `prompts.admin.bindings.update` |

### Open API (`/prompts/v3/api`)

| Method | Path | operationId |
| --- | --- | --- |
| GET | `/prompts/catalog` | `prompts.catalog.list` |

Public — no dual-token headers on read operations per `API_SPEC.md`.

## Security

- App and backend routes require `Authorization` + `Access-Token` (dual-token)
- Tenant isolation enforced in repository queries
- Render endpoint must not log full variable payloads in application logs (audit via `ai_prompt_usage.context` metadata only)

## SDK materialization

```bash
pnpm api:materialize
```

Outputs under `sdks/sdkwork-prompts-*-sdk/openapi/`.

## Verification

```bash
node tests/sdk/prompts-sdk.test.mjs
node tests/static/prompts-iam-path-alignment.test.mjs
```
