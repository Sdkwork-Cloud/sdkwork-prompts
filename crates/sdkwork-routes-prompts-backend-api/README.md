# SDKWork Router Prompts Backend API

Route crate for `/backend/v3/api/forum` with 36 routes.

## Implementation Status

- **Route descriptors**: 36 `RouteDescriptor` entries with method, path, operationId, surface="backend-api", auth_mode="dual-token", and tags.
- **Handlers**: Per-operation handler functions for admin operations (nodes, topics, moderation, sanctions, reputation, trust levels, badges, stats, search reindex, audit).
- **Mappers**: Request query param parsing, JSON response helpers, problem+json error mapping.
- **Path matching**: `find_route()` with template-based path matching.
- **Manifest**: `ManifestMetadata` with full SDKWork route manifest metadata.

Awaiting SDKWork Rust backend runtime and auth/context middleware.
