# SDKWork Prompts Backend SDK

Generated from `sdkwork-prompts-backend-api`.

## Contract

- Surface: `backend-api`
- Prefix: `/backend/v3/api`
- Auth: SDKWork dual-token in explicit backend-admin contexts.
- Dependency SDKs: appbase backend, Drive backend, search backend, messaging backend.

## Composed Facade

`PromptsBackendFacade` provides 30+ methods:
- Node CRUD, topic prefixes
- Topic CRUD + pin/feature/lock/move
- Moderation queue, cases, decisions
- Sanctions management
- Reputation rules, ledger
- Trust levels, badges
- Board/topic stats
- Search reindex
- Audit actions

## Generation

SDK output will be generated using the canonical SDKWork generator after route manifests and OpenAPI are reviewed.
