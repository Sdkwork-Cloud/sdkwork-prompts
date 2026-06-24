pub const SCHEMA_REGISTRY_PATH: &str = "specs/forum-database.schema.yaml";

pub const TABLES: &[&str] = &[
    "prm_space",
    "prm_node",
    "prm_board_profile",
    "prm_tag",
    "prm_topic_tag",
    "prm_topic_prefix",
    "prm_node_acl",
    "prm_topic",
    "prm_topic_revision",
    "prm_topic_reply",
    "prm_reply_revision",
    "prm_attachment",
    "prm_question_profile",
    "prm_poll",
    "prm_poll_option",
    "prm_poll_vote",
    "prm_reaction",
    "prm_vote",
    "prm_bookmark",
    "prm_subscription",
    "prm_read_state",
    "prm_notification_preference",
    "prm_member_profile",
    "prm_trust_level",
    "prm_privilege_grant",
    "prm_badge",
    "prm_user_badge",
    "prm_reputation_ledger",
    "prm_reputation_rule",
    "prm_report",
    "prm_moderation_queue_item",
    "prm_moderation_case",
    "prm_moderation_decision",
    "prm_moderation_policy",
    "prm_sanction",
    "prm_appeal",
    "prm_feed_item",
    "prm_public_topic_projection",
    "prm_topic_stats",
    "prm_board_stats",
    "prm_member_stats",
    "prm_search_document",
    "prm_outbox_event",
    "prm_inbox_event",
    "prm_idempotency_record",
];

pub const TABLE_GROUPS: &[(&str, &str)] = &[
    ("prm_space", "taxonomy"),
    ("prm_node", "taxonomy"),
    ("prm_board_profile", "taxonomy"),
    ("prm_tag", "taxonomy"),
    ("prm_topic_tag", "taxonomy"),
    ("prm_topic_prefix", "taxonomy"),
    ("prm_node_acl", "taxonomy"),
    ("prm_topic", "discussion"),
    ("prm_topic_revision", "discussion"),
    ("prm_topic_reply", "discussion"),
    ("prm_reply_revision", "discussion"),
    ("prm_attachment", "discussion"),
    ("prm_question_profile", "qa_poll"),
    ("prm_poll", "qa_poll"),
    ("prm_poll_option", "qa_poll"),
    ("prm_poll_vote", "qa_poll"),
    ("prm_reaction", "engagement"),
    ("prm_vote", "engagement"),
    ("prm_bookmark", "engagement"),
    ("prm_subscription", "engagement"),
    ("prm_read_state", "engagement"),
    ("prm_notification_preference", "engagement"),
    ("prm_member_profile", "member"),
    ("prm_trust_level", "member"),
    ("prm_privilege_grant", "member"),
    ("prm_badge", "member"),
    ("prm_user_badge", "member"),
    ("prm_reputation_ledger", "member"),
    ("prm_reputation_rule", "member"),
    ("prm_report", "moderation"),
    ("prm_moderation_queue_item", "moderation"),
    ("prm_moderation_case", "moderation"),
    ("prm_moderation_decision", "moderation"),
    ("prm_moderation_policy", "moderation"),
    ("prm_sanction", "moderation"),
    ("prm_appeal", "moderation"),
    ("prm_feed_item", "projection"),
    ("prm_public_topic_projection", "projection"),
    ("prm_topic_stats", "projection"),
    ("prm_board_stats", "projection"),
    ("prm_member_stats", "projection"),
    ("prm_search_document", "projection"),
    ("prm_outbox_event", "integration"),
    ("prm_inbox_event", "integration"),
    ("prm_idempotency_record", "integration"),
];

pub const TENANT_ENTITY_FIELD_SET: &[&str] = &[
    "id", "uuid", "tenant_id", "organization_id", "data_scope",
    "status", "version", "created_at", "updated_at", "deleted_at", "deleted_by",
];

pub const INTEGRATION_LOG_FIELD_SET: &[&str] = &[
    "id", "uuid", "tenant_id", "organization_id",
    "status", "version", "created_at", "updated_at",
];

pub fn ensure_known_table(table: &str) -> bool {
    TABLES.contains(&table)
}

pub fn table_group(table: &str) -> Option<&'static str> {
    TABLE_GROUPS.iter().find(|(t, _)| *t == table).map(|(_, g)| *g)
}

pub fn tables_in_group(group: &str) -> Vec<&'static str> {
    TABLE_GROUPS.iter().filter(|(_, g)| *g == group).map(|(t, _)| *t).collect()
}

pub fn is_tenant_scoped(table: &str) -> bool {
    ensure_known_table(table) && !matches!(table,
        "prm_outbox_event"
        | "prm_inbox_event"
        | "prm_idempotency_record"
    )
}

pub fn requires_idempotency(table: &str) -> bool {
    matches!(table,
        "prm_outbox_event"
        | "prm_inbox_event"
        | "prm_idempotency_record"
        | "prm_reputation_ledger"
    )
}
