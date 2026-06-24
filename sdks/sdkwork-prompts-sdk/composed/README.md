# Prompts Open SDK Composed Facade

Type-safe composed facade for forum open-api public read operations.

## Methods

- `listPublicTopics(siteSlug, params?)` - List public topics
- `listBoards(siteSlug, params?)` - List public boards
- `listBoardTopics(siteSlug, boardId, params?)` - List board topics
- `retrieveTopic(siteSlug, topicId)` - Retrieve public topic
- `retrieveTopicBySlug(siteSlug, topicSlug)` - Retrieve by slug
- `listTopicReplies(siteSlug, topicId, params?)` - List replies
- `listTags(siteSlug, params?)` - List tags
- `search(siteSlug, query, params?)` - Search public content

All operations are anonymous and do not require credentials.
