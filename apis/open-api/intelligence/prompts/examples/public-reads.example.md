# Prompts Open API Examples

Public read examples for open-api forum operations. No credentials required.

## List Boards

**Request:**
```http
GET /prompts/v3/api/sites/sdkwork-community/boards
```

**Response (200):**
```json
{
  "items": [
    {
      "id": 1001,
      "slug": "general",
      "name": "General Discussion",
      "description": "General topics about SDKWork",
      "topicCount": 150,
      "replyCount": 890,
      "lastActivityAt": "2026-06-13T10:00:00Z"
    }
  ],
  "nextCursor": null,
  "hasMore": false
}
```

## List Topics

**Request:**
```http
GET /prompts/v3/api/sites/sdkwork-community/topics?limit=20
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
      "slug": "how-to-use-sdkwork-prompts",
      "excerpt": "I want to learn how to create topics...",
      "authorDisplayName": "PromptsUser",
      "tagSlugs": ["help", "getting-started"],
      "replyCount": 5,
      "viewCount": 120,
      "lastActivityAt": "2026-06-13T10:05:00Z",
      "createdAt": "2026-06-13T10:00:00Z"
    }
  ],
  "nextCursor": null,
  "hasMore": false
}
```

## Retrieve Topic

**Request:**
```http
GET /prompts/v3/api/sites/sdkwork-community/topics/5001
```

**Response (200):**
```json
{
  "id": 5001,
  "uuid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "boardId": 1001,
  "title": "How to use SDKWork forum?",
  "slug": "how-to-use-sdkwork-prompts",
  "excerpt": "I want to learn how to create topics...",
  "authorDisplayName": "PromptsUser",
  "tagSlugs": ["help", "getting-started"],
  "replyCount": 5,
  "viewCount": 120,
  "lastActivityAt": "2026-06-13T10:05:00Z",
  "createdAt": "2026-06-13T10:00:00Z"
}
```

## Search

**Request:**
```http
GET /prompts/v3/api/sites/sdkwork-community/search?q=forum&limit=10
```

**Response (200):**
```json
{
  "items": [
    {
      "id": 5001,
      "sourceType": "topic",
      "sourceId": 5001,
      "title": "How to use SDKWork forum?",
      "excerpt": "I want to learn how to create topics...",
      "authorDisplayName": "PromptsUser",
      "boardId": 1001,
      "rankScore": "0.95"
    }
  ],
  "nextCursor": null,
  "hasMore": false
}
```
