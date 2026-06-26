# SDKWork Prompts API Server

- **Crate**: `sdkwork-prompts-api-server`
- **Domain**: intelligence
- **Capability**: prompts
- **Binary**: `sdkwork-prompts-api-server`
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
cargo build -p sdkwork-prompts-api-server
pnpm verify
```
