# Prompts Specs

This directory contains authored, reviewable contracts for the `intelligence/forum` capability.

## Contracts

- `component.spec.json`: SDKWork component contract for this root.
- `forum-database.schema.yaml`: database schema registry and table contracts.

## Schema Summary

- 45 tables across 8 groups (taxonomy, discussion, qa_poll, engagement, member, moderation, projection, integration)
- Standard field sets: tenant_entity (11 fields), integration_log (8 fields)
- Full constraint and index definitions per table
- Implementation guidance in `implementation_todo` fields

## DDL Generation

SQL DDL snapshots will be generated when the schema generator tool is connected. Target databases:
- PostgreSQL (primary)
- MySQL (secondary)
- SQLite (development/testing)
