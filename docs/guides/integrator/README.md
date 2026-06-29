# Integrator guide

## SDK families

| Package | Surface | Base path |
| --- | --- | --- |
| `@sdkwork/prompts-app-sdk` | app-api | `/app/v3/api` |
| `@sdkwork/prompts-backend-sdk` | backend-api | `/backend/v3/api` |
| `@sdkwork/prompts-sdk` | open-api | `/prompts/v3/api` |

Generated with `--standard-profile sdkwork-v3` — success responses unwrap `data` by default.

## Authentication

- **App / backend**: dual-token (`Authorization` + `Access-Token`) via SDK `TokenManager`
- **Open catalog**: public read; no auth headers on list operations

## Response envelope

Success:

```json
{ "code": 0, "data": { ... }, "traceId": "<uuid>" }
```

Errors: HTTP 4xx/5xx with `application/problem+json` (`ProblemDetail`).

## Materialization

```bash
pnpm api:materialize
pnpm api:sdkgen
```

Authority files: `apis/*/intelligence/prompts/openapi.yaml`

See [TECH-prompts-api-design.md](../../architecture/tech/TECH-prompts-api-design.md).
