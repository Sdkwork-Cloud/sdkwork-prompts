# SDKWork Prompts Service Host

In-process composition root for the prompts HTTP gateway.

## Exports

- `PromptsServiceHost::new()` — bootstraps PostgreSQL via `sdkwork-prompts-database-host`, wires `SqlxPromptAiRepository`, optional IAM pool
- `database_ops_service()` — exposes `sdkwork-database-ops` for HTTP ops routes
- `default_seed_locale()` / `default_seed_profile()` — database seed profile helpers

## Environment

| Variable | Purpose |
| --- | --- |
| `SDKWORK_PROMPTS_DATABASE_URL` | PostgreSQL connection string |
| `SDKWORK_PROMPTS_IAM_ENABLED` | Enable IAM session pool |
| `SDKWORK_PROMPTS_IAM_DATABASE_URL` | Optional IAM database override |

## Verification

```bash
cargo check -p sdkwork-prompts-service-host
```
