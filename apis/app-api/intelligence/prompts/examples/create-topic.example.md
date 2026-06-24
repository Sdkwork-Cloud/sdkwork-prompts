# Prompts App API Examples

Request/response examples for app-api forum operations.

## Create Topic

**Request:**
```http
POST /app/v3/api/prompts/topics
Content-Type: application/json
Authorization: Bearer <auth_token>
Access-Token: <access_token>

{
  "boardId": 1001,
  "title": "How to use SDKWork forum?",
  "bodyFormat": "markdown",
  "body": "I want to learn how to create topics in the SDKWork prompts.",
  "tagIds": [2001, 2002],
  "topicType": "discussion"
}
```

**Response (201):**
```json
{
  "id": 5001,
  "uuid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "boardId": 1001,
  "title": "How to use SDKWork forum?",
  "bodyFormat": "markdown",
  "body": "I want to learn how to create topics in the SDKWork prompts.",
  "topicType": "discussion",
  "moderationStatus": "visible",
  "visibility": "public",
  "version": 1,
  "createdAt": "2026-06-13T10:00:00Z",
  "updatedAt": "2026-06-13T10:00:00Z"
}
```

## List Topics

**Request:**
```http
GET /app/v3/api/prompts/boards/1001/topics?limit=20&sort=latest
Authorization: Bearer <auth_token>
Access-Token: <access_token>
```

**Response (200):**
```json
{
  "items": [
    {
      "id": 5001,
      "uuid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "boardId": 1001,
      "title": "How to use SDKWork forum?",
      "topicType": "discussion",
      "moderationStatus": "visible",
      "version": 1,
      "createdAt": "2026-06-13T10:00:00Z",
      "updatedAt": "2026-06-13T10:00:00Z"
    }
  ],
  "nextCursor": null,
  "hasMore": false
}
```

## Create Reply

**Request:**
```http
POST /app/v3/api/prompts/topics/5001/replies
Content-Type: application/json
Authorization: Bearer <auth_token>
Access-Token: <access_token>

{
  "bodyFormat": "markdown",
  "body": "You can create topics by clicking the 'New Topic' button."
}
```

**Response (201):**
```json
{
  "id": 6001,
  "uuid": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
  "topicId": 5001,
  "replyNo": 1,
  "bodyFormat": "markdown",
  "body": "You can create topics by clicking the 'New Topic' button.",
  "moderationStatus": "visible",
  "attachmentCount": 0,
  "version": 1,
  "createdAt": "2026-06-13T10:05:00Z",
  "updatedAt": "2026-06-13T10:05:00Z"
}
```

## Search

**Request:**
```http
GET /app/v3/api/prompts/search?q=SDKWork&limit=10
Authorization: Bearer <auth_token>
Access-Token: <access_token>
```

**Response (200):**
```json
{
  "items": [
    {
      "id": 5001,
      "uuid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "sourceType": "topic",
      "sourceId": 5001,
      "title": "How to use SDKWork forum?",
      "visibility": "public",
      "indexStatus": "indexed"
    }
  ],
  "nextCursor": null,
  "hasMore": false
}
```
