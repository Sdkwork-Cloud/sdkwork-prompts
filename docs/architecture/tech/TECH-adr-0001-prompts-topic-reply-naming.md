> Migrated from `docs/adr/ADR-0001-prompts-topic-reply-naming.md` on 2026-06-24.
> Owner: SDKWork maintainers

## Status

Accepted.

## Context

Prompts systems often use several names for the primary discussion unit. SDKWork needs names that are clear in APIs, database contracts, SDK resources, Rust modules, and documentation.

## Decision

Prompts public contracts use `topic` and `reply` terminology.

`thread` is forbidden in database table names, API paths, SDK resource names, and route crate public resources because it collides with runtime/concurrency terminology.

The route crate capability remains `forum` because it names the business capability, not a discussion object.

## Consequences

- Tables use names such as `prm_topic` and `prm_topic_reply`.
- APIs use paths such as `/topics/{topicId}/replies`.
- SDK resources use operationIds such as `topics.replies.list`.
- Docs may mention the rejected term only in this ADR or migration notes.

