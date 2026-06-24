# Prompts Backend API Examples

Admin examples for backend-api forum operations.

## Create Moderation Decision

**Request:**
```http
POST /backend/v3/api/prompts/moderation/cases/7001/decisions
Content-Type: application/json
Authorization: Bearer <auth_token>
Access-Token: <access_token>

{
  "decisionAction": "hide",
  "reasonCode": "spam",
  "note": "Content identified as spam by automated policy."
}
```

**Response (201):**
```json
{
  "id": 8001,
  "uuid": "c3d4e5f6-a7b8-9012-cdef-123456789012",
  "caseId": 7001,
  "decisionAction": "hide",
  "reasonCode": "spam",
  "note": "Content identified as spam by automated policy.",
  "decidedBy": 100,
  "createdAt": "2026-06-13T10:00:00Z"
}
```

## List Moderation Queue

**Request:**
```http
GET /backend/v3/api/prompts/moderation/queue?status=open&severity=high&limit=20
Authorization: Bearer <auth_token>
Access-Token: <access_token>
```

**Response (200):**
```json
{
  "items": [
    {
      "id": 7001,
      "uuid": "d4e5f6a7-b8c9-0123-defa-234567890123",
      "caseNo": "MOD-2026-0001",
      "targetType": "topic",
      "targetId": 5001,
      "caseStatus": "open",
      "severity": "high",
      "openedBy": 100,
      "createdAt": "2026-06-13T09:00:00Z"
    }
  ],
  "nextCursor": null,
  "hasMore": false
}
```

## Create Sanction

**Request:**
```http
POST /backend/v3/api/prompts/sanctions
Content-Type: application/json
Authorization: Bearer <auth_token>
Access-Token: <access_token>

{
  "userId": 2001,
  "caseId": 7001,
  "sanctionType": "suspend",
  "reasonCode": "repeated_spam",
  "expiresAt": "2026-06-20T00:00:00Z"
}
```

**Response (201):**
```json
{
  "id": 9001,
  "uuid": "e5f6a7b8-c9d0-1234-efab-345678901234",
  "userId": 2001,
  "caseId": 7001,
  "sanctionType": "suspend",
  "reasonCode": "repeated_spam",
  "startsAt": "2026-06-13T10:00:00Z",
  "expiresAt": "2026-06-20T00:00:00Z",
  "status": "active",
  "createdAt": "2026-06-13T10:00:00Z",
  "updatedAt": "2026-06-13T10:00:00Z"
}
```

## Rebuild Search Index

**Request:**
```http
POST /backend/v3/api/prompts/search/reindex
Content-Type: application/json
Authorization: Bearer <auth_token>
Access-Token: <access_token>

{
  "scope": "board",
  "boardId": 1001
}
```

**Response (202):**
```json
{
  "success": true
}
```

## Create Reputation Rule

**Request:**
```http
POST /backend/v3/api/prompts/reputation/rules
Content-Type: application/json
Authorization: Bearer <auth_token>
Access-Token: <access_token>

{
  "code": "topic_created",
  "eventType": "prompts.topic.created",
  "points": 5,
  "dailyLimit": 10
}
```

**Response (201):**
```json
{
  "id": 10001,
  "uuid": "f6a7b8c9-d0e1-2345-fabc-456789012345",
  "code": "topic_created",
  "eventType": "prompts.topic.created",
  "points": 5,
  "status": "active",
  "createdAt": "2026-06-13T10:00:00Z",
  "updatedAt": "2026-06-13T10:00:00Z"
}
```
