# Runbooks

## API unavailable

1. Check process: `sdkwork-prompts-standalone-gateway` listening on configured bind (default `0.0.0.0:8080`)
2. Check database: `pnpm db:status` and `SDKWORK_PROMPTS_DATABASE_URL`
3. Review logs: `RUST_LOG=info` (set `debug` for SQL tracing)

## Database drift

```bash
pnpm db:drift:check
pnpm db:migrate
```

## Contract regression

```bash
pnpm verify
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
```

## IAM auth failures

- Confirm `SDKWORK_PROMPTS_REQUIRE_AUTH` and `SDKWORK_PROMPTS_IAM_ENABLED` match deployment profile
- Verify dual-token headers on app/backend routes
- Optional: `SDKWORK_PROMPTS_IAM_DATABASE_URL` when IAM data is not in prompts DB
