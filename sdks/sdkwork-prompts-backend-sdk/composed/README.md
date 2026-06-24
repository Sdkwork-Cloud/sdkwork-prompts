# Prompts Backend SDK Composed Facade

Type-safe composed facade for forum backend-api admin operations.

## Methods

### Moderation
- `createModerationDecision(caseId, body)` - Create decision with notification
- `listModerationQueue(params?)` - List moderation queue
- `listModerationCases(params?)` - List moderation cases
- `createModerationCase(body)` - Create moderation case
- `retrieveModerationCase(caseId)` - Retrieve case details

### Node Management
- `listNodes(params?)` - List forum nodes
- `createNode(body)` - Create category/board
- `updateNode(nodeId, body)` - Update node
- `deleteNode(nodeId)` - Delete node

### Topic Management
- `listTopics(params?)` - List all topics
- `retrieveTopic(topicId)` - Retrieve topic
- `updateTopic(topicId, body)` - Update topic
- `deleteTopic(topicId)` - Delete topic
- `pinTopic(topicId)` / `unpinTopic(topicId)` - Pin/unpin
- `featureTopic(topicId)` / `unfeatureTopic(topicId)` - Feature/unfeature
- `lockTopic(topicId)` / `unlockTopic(topicId)` - Lock/unlock
- `moveTopic(topicId, targetBoardId)` - Move to another board

### Sanctions
- `listSanctions(params?)` - List sanctions
- `createSanction(body)` - Create sanction
- `updateSanction(sanctionId, body)` - Update sanction

### Reputation & Trust
- `listReputationRules(params?)` / `createReputationRule(body)` - Manage rules
- `listReputationLedger(params?)` - View ledger
- `listTrustLevels(params?)` / `createTrustLevel(body)` - Manage levels

### Badges
- `listBadges(params?)` / `createBadge(body)` - Manage badges

### Stats & Maintenance
- `listBoardStats(params?)` / `listTopicStats(params?)` - View stats
- `rebuildSearchProjection(body?)` - Rebuild search index
- `listAuditActions(params?)` - View audit trail
- `listTopicPrefixes(params?)` / `createTopicPrefix(body)` - Manage prefixes
