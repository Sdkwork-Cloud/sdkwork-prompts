# Prompts App SDK Composed Facade

Type-safe composed facade for forum app-api operations.

## Methods

- `listBoardTopics(boardId, params?)` - List topics in a board
- `createTopic(body)` - Create a topic with Drive attachment validation
- `createReply(topicId, body)` - Create a reply with auth check
- `retrieveTopic(topicId)` - Retrieve topic details
- `updateTopic(topicId, body)` - Update topic
- `deleteTopic(topicId)` - Delete topic
- `listReplies(topicId, params?)` - List replies for a topic
- `updateReply(replyId, body)` - Update reply
- `deleteReply(replyId)` - Delete reply
- `listTopicRevisions(topicId, params?)` - List topic edit history
- `listReplyRevisions(replyId, params?)` - List reply edit history
- `acceptReply(topicId, replyId)` - Accept answer (question topics)
- `clearAcceptedReply(topicId)` - Clear accepted answer
- `votePoll(pollId, optionIds)` - Vote in a poll
- `createReaction(targetType, targetId, reactionType)` - Add reaction
- `createVote(targetType, targetId, voteValue)` - Up/down vote
- `updateBookmark(targetType, targetId, note?)` - Bookmark content
- `updateReadState(topicId, lastReadReplyId?)` - Mark as read
- `createReport(body)` - Report content
- `listFeed(params?)` - List personalized feed
- `search(query, params?)` - Search forum content
- `listNodeTree(params?)` - List category/board tree
