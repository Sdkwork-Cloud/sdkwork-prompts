# SDKWork Prompts App SDK

Generated from `sdkwork-prompts-app-api`.

## Contract

- Surface: `app-api`
- Prefix: `/app/v3/api`
- Auth: SDKWork dual-token through the application global TokenManager.
- Dependency SDKs: appbase app, Drive app, search app, messaging app.

## Composed Facade

`PromptsAppFacade` provides 22 methods:
- Node tree listing
- Topic CRUD + revisions
- Reply CRUD + revisions
- Accepted reply (Q&A)
- Poll votes, reactions, votes
- Bookmarks, read state
- Reports, feed, search

## Generation

SDK output will be generated using the canonical SDKWork generator after OpenAPI review.
