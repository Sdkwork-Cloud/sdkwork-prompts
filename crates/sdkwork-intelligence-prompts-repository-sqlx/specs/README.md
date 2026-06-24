# Repository Specs

Component spec for `sdkwork-intelligence-prompts-repository-sqlx`.

- **Crate type**: SQLx repository implementation
- **Domain**: intelligence
- **Capability**: forum
- **Tables**: 46 tables across 8 groups (taxonomy, discussion, qa_poll, engagement, member, moderation, projection, integration)
- **Schema registry**: `specs/forum-database.schema.yaml`
- **Implementation**: `SqlxPromptsRepository` implements all `PromptsRepository` methods against PostgreSQL with snowflake ids via `sdkwork-id-core`.
