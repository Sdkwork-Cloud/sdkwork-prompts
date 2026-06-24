# SDKWork Prompts Open SDK

Generated from `sdkwork-prompts-open-api`.

## Contract

- Surface: `open-api`
- Prefix: `/prompts/v3/api`
- Auth: anonymous public reads in the current contract.
- Dependencies: none.

This SDK must not use the application TokenManager for anonymous operations.

## Composed Facade

`PromptsOpenFacade` provides 8 methods:
- `listPublicTopics(siteSlug, params?)` - List public topics
- `listBoards(siteSlug, params?)` - List public boards
- `listBoardTopics(siteSlug, boardId, params?)` - List board topics
- `retrieveTopic(siteSlug, topicId)` - Retrieve public topic
- `retrieveTopicBySlug(siteSlug, topicSlug)` - Retrieve by slug
- `listTopicReplies(siteSlug, topicId, params?)` - List replies
- `listTags(siteSlug, params?)` - List tags
- `search(siteSlug, query, params?)` - Search public content

## API Key Credential

API-key credential provider will be added only if future protected Open API write operations are approved. Current contract is anonymous-only.
