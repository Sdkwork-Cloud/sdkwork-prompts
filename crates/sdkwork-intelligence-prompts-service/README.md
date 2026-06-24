# SDKWork Communication Prompts Service

Domain service boundary for `intelligence/forum`.

This crate owns command validation, orchestration, repository ports, outbox decisions, and domain errors. It does not own HTTP route construction or SQL adapter details.

## Implementation Status

- **Command validation**: Implemented. All service methods validate input fields, enum values, limits, and required fields.
- **Repository ports**: `PromptsRepository` trait with 40+ methods. `SqlxPromptsRepository` provides SQLx-backed implementations for all trait methods.
- **Integration ports**: `PromptsDrivePort`, `PromptsSearchPort`, `PromptsNotificationPort` traits with `NoopPrompts*Port` (silent success) and `LoggingPrompts*Port` (stderr logging) implementations.
- **Error types**: 12 error variants with HTTP status code mapping.
- **Value objects**: Typed IDs (i64), enums for ModerationStatus, TopicType, Visibility, NodeType, BodyFormat, DataScope.
