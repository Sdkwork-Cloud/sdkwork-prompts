# SDKWork Prompts API Server

- **Crate**: `sdkwork-api-prompts-standalone-gateway`
- **Domain**: intelligence
- **Capability**: prompts
- **Binary**: `sdkwork-api-prompts-standalone-gateway`
- **Routes**: 18 (app-api 8, backend-api 9, open-api 1)
- **Exports**: `compose_prompts_api_routes()`, `PromptsRouteInfo`, `route_count()`, `find_route()`

## Handlers

| Module | Surface |
| --- | --- |
| `routes/ai_app_prompts.rs` | app-api templates |
| `routes/ai_app_agent_templates.rs` | app-api agent templates |
| `routes/ai_prompts.rs` | backend-api admin |
| `routes/open.rs` | open-api catalog |

## Verification

```bash
cargo build -p sdkwork-api-prompts-standalone-gateway
pnpm verify
```
