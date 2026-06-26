# SDKWork Router Prompts Open API

Route crate for `/prompts/v3/api` with 8 public read routes.

## Implementation Status

- **Route descriptors**: 8 `RouteDescriptor` entries with method, path, operationId, surface="open-api", auth_mode="public", and tags.
- **Handlers**: Per-operation handler functions for public reads (boards list, topics list/retrieve, replies list, tags list, search).
- **Mappers**: Request query param parsing, JSON response helpers, problem+json error mapping.
- **Path matching**: `find_route()` with template-based path matching including `{siteSlug}` and `{topicSlug}`.
- **Manifest**: `ManifestMetadata` with full SDKWork route manifest metadata.

Open API public reads are anonymous and do not use SDKWork dual-token headers.
