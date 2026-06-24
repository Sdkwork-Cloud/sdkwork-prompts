# Prompts API Design

## API Surfaces

App API:
- Prefix: `/app/v3/api/forum`.
- SDK family: `sdkwork-prompts-app-sdk`.
- Security: SDKWork dual-token through generated SDK TokenManager.
- Purpose: authenticated user-facing forum operations.

Backend API:
- Prefix: `/backend/v3/api/forum`.
- SDK family: `sdkwork-prompts-backend-sdk`.
- Security: SDKWork dual-token through explicit backend-admin runtime.
- Purpose: operator, moderation, taxonomy, reputation, stats, search, and audit operations.

Open API:
- Prefix: `/prompts/v3/api`.
- SDK family: `sdkwork-prompts-sdk`.
- Security: current operations are anonymous public reads.
- Purpose: public boards, topics, replies, tags, and search.

## Open API Simplicity

Open API public operations use:

```yaml
security: []
x-sdkwork-auth-mode: anonymous
x-sdkwork-public: true
```

They do not declare SDKWork internal context headers, tenant headers, organization headers, dual-token headers, or generated SDK manual auth headers.

If future write/integration operations are required, they must use exactly one external API key security scheme and remain separate from anonymous public read operations.

## Resource Naming

The API uses `topic` and `reply`. The term rejected in ADR-0001 must not appear in paths, operationIds, schemas, SDK resources, or route manifests.

## SDK Operation Shape

OperationIds are dotted resource-style names:
- `topics.list`
- `topics.replies.create`
- `questions.acceptedReply.update`
- `moderation.cases.decisions.create`
- `search.query`

This maps cleanly to nested SDK clients while keeping HTTP paths stable.

## Write Command Examples

### Create Topic

```json
{
  "boardId": 1001,
  "title": "How to use SDKWork forum?",
  "bodyFormat": "markdown",
  "body": "I want to learn how to create topics.",
  "tagIds": [2001, 2002],
  "topicType": "discussion"
}
```

Service validation:
- `title` must not be empty, max 240 characters
- `body` must not be empty
- `body_format` must be one of: `markdown`, `html_sanitized`, `rich_text_json`
- `board_id` must reference an existing board with `topic_create_mode` != `closed`

### Create Reply

```json
{
  "parentReplyId": null,
  "bodyFormat": "markdown",
  "body": "You can create topics by clicking 'New Topic'."
}
```

Service validation:
- `body` must not be empty
- `body_format` must be valid
- Topic must not be locked
- User must not be sanctioned with `restrict_posting`

### Create Moderation Decision

```json
{
  "decisionAction": "hide",
  "reasonCode": "spam",
  "note": "Content identified as spam by automated policy."
}
```

Service validation:
- `reason_code` must not be empty
- `decision_action` must be one of: `dismiss`, `hide`, `restore`, `lock`, `move`, `sanction`, `escalate`

### Create Sanction

```json
{
  "userId": 2001,
  "caseId": 7001,
  "sanctionType": "suspend",
  "reasonCode": "repeated_spam",
  "expiresAt": "2026-06-20T00:00:00Z"
}
```

Service validation:
- `sanction_type` must be one of: `mute`, `suspend`, `restrict_posting`, `ban`
- `reason_code` must not be empty

## Idempotency

Write commands support idempotency through the `Idempotency-Key` header:
- `topics.create`, `replies.create`, `reports.create` - Client-supplied idempotency key
- `moderation.cases.decisions.create` - Operator-supplied idempotency key
- `sanctions.create` - System-generated from target+type+user

The repository layer stores idempotency records in `prm_idempotency_record` with:
- `idempotency_key` - Client or server key
- `request_hash` - SHA-256 of canonical request body
- `operation_id` - OpenAPI operationId
- `response_status` and `response_body_json` - Stored successful response

Duplicate requests with the same key and same hash return the stored response. Requests with the same key but different hash return 409 Conflict.

## Contract Tests

Contract validation is implemented in:
- `tests/contract/prompts-contract.test.mjs` - Validates route manifests, assemblies, auth modes
- `tests/schema/prompts-schema.test.mjs` - Validates database schema completeness
- `tests/sdk/forum-sdk.test.mjs` - Validates sdkgen configs and composed facades
- `tests/static/prompts-contract-boundary.test.mjs` - Validates forbidden terms and boundary rules

## Response Envelope

Current responses use direct JSON without a wrapper envelope:

```json
// Single resource
{ "id": 5001, "title": "...", ... }

// Paginated list
{ "items": [...], "nextCursor": "...", "hasMore": true }

// Command result
{ "success": true, "id": 5001, "uuid": "..." }
```

If the SDKWork generator requires a stricter page wrapper, the envelope will be documented here.
