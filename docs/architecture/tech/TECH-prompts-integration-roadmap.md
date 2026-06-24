> Migrated from `docs/prompts-integration-roadmap.md` on 2026-06-24.
> Owner: SDKWork maintainers

## Foundation Dependencies

Current metadata avoids declaring unresolved SDK dependencies as generation inputs. The following integrations must be added once their local SDK family paths and authority contracts are verified:

- Appbase app SDK: login/session/current user/workspace context for app clients.
- Appbase backend SDK: backend-admin IAM and permission management.
- Drive app/backend SDKs: attachment upload sessions, media resource selection, and download grants.
- Search backend SDK: full reindex and incremental indexing adapters.
- Messaging or notification SDK: subscription and moderation notification delivery.

## Integration Status

### SDK Dependencies

Planned `sdkDependencies` entries for each SDK family:

```yaml
# sdkwork-prompts-app-sdk
sdkDependencies:
  - sdkFamily: sdkwork-appbase-app-sdk
    authority: sdkwork-appbase-app-api
    purpose: IAM login, session, current user context
  - sdkFamily: sdkwork-drive-app-sdk
    authority: sdkwork-drive-app-api
    purpose: Attachment upload sessions and media resource selection

# sdkwork-prompts-backend-sdk
sdkDependencies:
  - sdkFamily: sdkwork-appbase-backend-sdk
    authority: sdkwork-appbase-backend-api
    purpose: IAM permission management and operator context
  - sdkFamily: sdkwork-drive-backend-sdk
    authority: sdkwork-drive-backend-api
    purpose: Download grants and media lifecycle management
  - sdkFamily: sdkwork-search-backend-sdk
    authority: sdkwork-search-backend-api
    purpose: Full reindex and incremental indexing

# sdkwork-prompts-sdk (open)
sdkDependencies: []
# Open API is anonymous public reads only; no external SDK dependencies required.
```

### Service Ports (Implemented)

| Port | Trait | Status | Implementations |
|------|-------|--------|-----------------|
| Drive | `PromptsDrivePort` | Partial | `NoopPromptsDrivePort`, `LoggingPromptsDrivePort` (awaiting Drive SDK) |
| Search | `PromptsSearchPort` | Implemented | `HttpPromptsSearchPort` (`sdkwork-search-backend-api` upsert/delete/rebuild), `LoggingPromptsSearchPort`, `NoopPromptsSearchPort` |
| Notification | `PromptsNotificationPort` | Partial | `HttpPromptsNotificationPort` (generic HTTP), `LoggingPromptsNotificationPort`, `NoopPromptsNotificationPort` |

### IAM Request Context

When `SDKWORK_PROMPTS_IAM_ENABLED=true`, the forum API server resolves `Authorization` + `Access-Token` against `iam_session` via `sdkwork-iam-web-adapter`. The resolved tenant/org/user ids populate `PromptsRequestContext` before handlers run. Set `SDKWORK_PROMPTS_IAM_STRICT=true` to reject invalid sessions on app/backend forum routes instead of falling back to header/env defaults.

Use `SDKWORK_PROMPTS_IAM_DATABASE_URL` when IAM sessions live outside the forum database module; otherwise the forum PostgreSQL pool is reused.

### Drive Media Grants

`PromptsDrivePort` provides:
- `validate_media_reference(media_resource_id)` - Verify tenant scope, ownership, scan status, and lifecycle
- `create_download_grant(media_resource_id)` - Create scoped download grant for attachments

Awaiting `sdkwork-drive-app-sdk` dependency resolution for real implementation.

### Notification Event Publisher

`PromptsNotificationPort` provides:
- `publish_prm_event(event_type, aggregate_id)` - Generic forum event publication
- `publish_moderation_alert(case_id, severity)` - Moderation alert to operators
- `publish_subscription_notification(user_id, event_type, target_id)` - Subscription delivery

Awaiting `sdkwork-messaging-sdk` dependency resolution for real implementation.

### Search Indexing Adapter

`PromptsSearchPort` provides:
- `index_document(source_type, source_id)` - Upsert via `PUT /backend/v3/api/search/indexes/{indexId}/documents/{documentId}`
- `delete_document(source_type, source_id)` - Remove via `DELETE` on the same path
- `rebuild_index(board_id)` - Trigger `POST /backend/v3/api/search/jobs/rebuild` for the configured index

Configure `SDKWORK_PROMPTS_SEARCH_URL`, `SDKWORK_PROMPTS_SEARCH_INDEX_ID`, and private bootstrap `SDKWORK_ACCESS_TOKEN` for backend search calls. Board-scoped rebuild remains a forum-side concern until search exposes scoped rebuild filters.

### Appbase Permission Mapping

Prompts permission codes planned:

| Code | Description | Surface |
|------|-------------|---------|
| `prompts.topics.create` | Create topics | app-api |
| `prompts.topics.read` | Read topics | app-api, open-api |
| `prompts.topics.update` | Update own topics | app-api |
| `prompts.topics.delete` | Delete own topics | app-api |
| `prompts.replies.create` | Create replies | app-api |
| `prompts.replies.read` | Read replies | app-api, open-api |
| `prompts.replies.update` | Update own replies | app-api |
| `prompts.replies.delete` | Delete own replies | app-api |
| `prompts.moderation.read` | Read moderation queue | backend-api |
| `prompts.moderation.write` | Create decisions | backend-api |
| `prompts.admin.nodes` | Manage taxonomy | backend-api |
| `prompts.admin.reputation` | Manage reputation rules | backend-api |
| `prompts.admin.badges` | Manage badges | backend-api |

Awaiting `sdkwork-appbase-backend-sdk` dependency resolution for permission enforcement on backend routes. IAM session resolution is implemented in `sdkwork-prompts-api-server` when enabled via env.

