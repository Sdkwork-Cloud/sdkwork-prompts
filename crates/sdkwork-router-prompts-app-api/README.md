# SDKWork Router Prompts App API

Route crate for `/app/v3/api/forum` with 22 routes.

## Implementation Status

- **Route descriptors**: 22 `RouteDescriptor` entries with method, path, operationId, surface, auth_mode, and tags.
- **Handlers**: Contract-test placeholders returning `PromptsRouteError::not_implemented`. Runtime traffic is served by `sdkwork-prompts-api-server/src/routes/app.rs`.
- **Mappers**: Request query param parsing, JSON response helpers, problem+json error mapping.
- **Path matching**: `find_route()` with template-based path matching for `{param}` segments.
- **Manifest**: `ManifestMetadata` with schemaVersion, kind, packageName, surface, owner, domain, capability, apiAuthority, sdkFamily, prefix.
