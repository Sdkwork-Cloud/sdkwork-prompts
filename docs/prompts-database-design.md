# Prompts Database Design

## Design Baseline

The authoritative machine-readable contract is `specs/forum-database.schema.yaml`.

Database naming follows `DATABASE_SPEC.md`:
- Table prefix is `prm_`.
- Multi-tenant tables use `tenant_id`, `organization_id`, `data_scope`, `status`, `version`, `created_at`, and `updated_at`.
- Stable external ids use `uuid`.
- Soft delete uses `deleted_at` and `deleted_by`.
- Event consistency uses `prm_outbox_event`, `prm_inbox_event`, and `prm_idempotency_record`.

The domain vocabulary is `topic` and `reply`. The rejected ambiguous term is documented in ADR-0001 only.

## Table Groups

Taxonomy:
- `prm_space`: top-level forum area.
- `prm_node`: category/board tree.
- `prm_board_profile`: board behavior and rules.
- `prm_tag`, `prm_topic_tag`: reusable tags and topic relations.
- `prm_topic_prefix`: board-local labels.
- `prm_node_acl`: forum-specific ACL overrides over IAM principals.

Discussion:
- `prm_topic`: source-of-truth topic.
- `prm_topic_revision`: immutable topic edit history.
- `prm_topic_reply`: source-of-truth reply.
- `prm_reply_revision`: immutable reply edit history.
- `prm_attachment`: Drive/media references bound to topics or replies.

Q&A and poll:
- `prm_question_profile`: accepted answer and bounty state.
- `prm_poll`, `prm_poll_option`, `prm_poll_vote`: poll state and votes.

Engagement:
- `prm_reaction`, `prm_vote`, `prm_bookmark`, `prm_subscription`, `prm_read_state`, `prm_notification_preference`.

Member and reputation:
- `prm_member_profile`, `prm_trust_level`, `prm_privilege_grant`.
- `prm_badge`, `prm_user_badge`.
- `prm_reputation_rule`, `prm_reputation_ledger`.

Moderation:
- `prm_report`, `prm_moderation_queue_item`, `prm_moderation_case`, `prm_moderation_decision`, `prm_moderation_policy`, `prm_sanction`, `prm_appeal`.

Projection:
- `prm_feed_item`, `prm_public_topic_projection`, `prm_topic_stats`, `prm_board_stats`, `prm_member_stats`, `prm_search_document`.

Integration:
- `prm_outbox_event`, `prm_inbox_event`, `prm_idempotency_record`.

## Core Query Patterns

- Board topic list: `prm_topic(tenant_id, board_id, moderation_status, last_activity_at, id)`.
- Topic replies: `prm_topic_reply(tenant_id, topic_id, moderation_status, created_at, id)`.
- Public Open API list: `prm_public_topic_projection(tenant_id, site_slug, status, updated_at, id)`.
- Moderation queue: `prm_moderation_queue_item(tenant_id, queue_status, severity, due_at, id)`.
- Search staging: `prm_search_document(tenant_id, index_status, updated_at, id)`.
- Outbox polling: `prm_outbox_event(status, next_attempt_at, id)`.

## Ownership Boundaries

IAM/appbase owns users, tenants, organizations, sessions, roles, and API keys. Prompts stores stable ids and never duplicates IAM source tables.

Drive owns file bytes, upload sessions, download grants, and media lifecycle. Prompts stores Drive/media references in `prm_attachment`, `prm_space`, `prm_node`, `prm_member_profile`, and `prm_badge`.

Search owns the external index. Prompts owns the staging projection `prm_search_document`.

Notification providers own delivery. Prompts owns subscription and preference state plus outbox events.

## Implementation Priorities

1. Generate migrations for taxonomy, discussion, integration, and projections needed by topic/reply list/create.
2. Implement idempotency and outbox transaction helpers before write endpoints.
3. Implement revisions for topic/reply update before exposing update APIs.
4. Implement moderation queue and decisions before automated policy rules.
5. Implement stats/search rebuild jobs after core write paths are stable.

## DDL Generation

SQL DDL snapshots will be generated from `specs/forum-database.schema.yaml` when the schema generator tool is connected.

### Planned DDL Targets

- **PostgreSQL** (primary): `deployments/sql/postgresql/V0.1.0__prm_foundation.sql`
- **MySQL** (secondary): `deployments/sql/mysql/V0.1.0__prm_foundation.sql`
- **SQLite** (development): `deployments/sql/sqlite/V0.1.0__prm_foundation.sql`

### Schema Contract Summary

- 45 tables across 8 groups
- Standard field sets: tenant_entity (11 fields), integration_log (8 fields)
- 4 required indexes minimum per tenant table
- Unique constraints on uuid, business keys, and tenant-scoped identifiers
- Soft delete via deleted_at/deleted_by on all tenant_entity tables
