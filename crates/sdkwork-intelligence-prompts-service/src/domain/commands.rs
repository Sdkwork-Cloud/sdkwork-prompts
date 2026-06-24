use serde_json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNodeTreeCommand {
    pub space_id: Option<i64>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTopicsCommand {
    pub board_id: Option<i64>,
    pub cursor: Option<String>,
    pub limit: u16,
    pub sort: Option<String>,
    pub status_filter: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTopicCommand {
    pub board_id: i64,
    pub title: String,
    pub body_format: String,
    pub body: String,
    pub tag_ids: Vec<i64>,
    pub prefix_id: Option<i64>,
    pub topic_type: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateTopicCommand {
    pub topic_id: i64,
    pub title: Option<String>,
    pub body_format: Option<String>,
    pub body: Option<String>,
    pub edit_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteTopicCommand {
    pub topic_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListRepliesCommand {
    pub topic_id: i64,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateReplyCommand {
    pub topic_id: i64,
    pub parent_reply_id: Option<i64>,
    pub body_format: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateReplyCommand {
    pub reply_id: i64,
    pub body_format: Option<String>,
    pub body: Option<String>,
    pub edit_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteReplyCommand {
    pub reply_id: i64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptReplyCommand {
    pub topic_id: i64,
    pub reply_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClearAcceptedReplyCommand {
    pub topic_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateReportCommand {
    pub target_type: String,
    pub target_id: i64,
    pub reason_code: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFeedCommand {
    pub feed_type: Option<String>,
    pub feed_owner_id: Option<String>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuerySearchCommand {
    pub query: String,
    pub board_id: Option<i64>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListModerationQueueCommand {
    pub status_filter: Option<String>,
    pub severity_filter: Option<String>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateModerationDecisionCommand {
    pub case_id: i64,
    pub decision_action: String,
    pub reason_code: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildSearchProjectionCommand {
    pub scope: Option<String>,
    pub board_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebuildStatsCommand {
    pub scope: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishOutboxCommand {
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanoutNotificationsCommand {
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTopicRevisionsCommand {
    pub topic_id: i64,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListReplyRevisionsCommand {
    pub reply_id: i64,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePollVoteCommand {
    pub poll_id: i64,
    pub option_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateReactionCommand {
    pub target_type: String,
    pub target_id: i64,
    pub reaction_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateVoteCommand {
    pub target_type: String,
    pub target_id: i64,
    pub vote_value: i32,
    pub reason_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateBookmarkCommand {
    pub target_type: String,
    pub target_id: i64,
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateReadStateCommand {
    pub topic_id: i64,
    pub last_read_reply_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinTopicCommand {
    pub topic_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureTopicCommand {
    pub topic_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LockTopicCommand {
    pub topic_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveTopicCommand {
    pub topic_id: i64,
    pub target_board_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateNodeCommand {
    pub space_id: i64,
    pub parent_id: Option<i64>,
    pub node_type: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateNodeCommand {
    pub node_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteNodeCommand {
    pub node_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListModerationCasesCommand {
    pub status_filter: Option<String>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateModerationCaseCommand {
    pub target_type: String,
    pub target_id: i64,
    pub severity: String,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveModerationCaseCommand {
    pub case_id: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSanctionsCommand {
    pub user_id: Option<i64>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSanctionCommand {
    pub user_id: i64,
    pub case_id: Option<i64>,
    pub decision_id: Option<i64>,
    pub sanction_type: String,
    pub reason_code: String,
    pub starts_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSanctionCommand {
    pub sanction_id: i64,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListReputationRulesCommand {
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateReputationRuleCommand {
    pub code: String,
    pub event_type: String,
    pub points: i64,
    pub daily_limit: Option<i64>,
    pub rule_json: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListReputationLedgerCommand {
    pub user_id: Option<i64>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTrustLevelsCommand {
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTrustLevelCommand {
    pub level_no: i32,
    pub code: String,
    pub name: String,
    pub threshold_rules: serde_json::Value,
    pub privileges: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBadgesCommand {
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateBadgeCommand {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub grant_mode: String,
    pub icon_media_id: Option<String>,
    pub rule_json: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBoardStatsCommand {
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTopicStatsCommand {
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAuditActionCommand {
    pub action: String,
    pub target_type: String,
    pub target_id: i64,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListAuditActionsCommand {
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNodesCommand {
    pub space_id: Option<i64>,
    pub node_type: Option<String>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTagsCommand {
    pub space_id: Option<i64>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetrieveTopicBySlugCommand {
    pub slug: String,
    pub board_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTopicPrefixesCommand {
    pub board_id: Option<i64>,
    pub cursor: Option<String>,
    pub limit: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTopicPrefixCommand {
    pub board_id: i64,
    pub code: String,
    pub label: String,
    pub color: Option<String>,
    pub sort_order: i32,
    pub required_trust_level: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSpaceCommand {
    pub code: String,
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub visibility: String,
    pub default_locale: Option<String>,
    pub settings: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSpaceCommand {
    pub space_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateAttachmentCommand {
    pub owner_type: String,
    pub owner_id: i64,
    pub drive_space_id: String,
    pub drive_node_id: String,
    pub media_resource_id: Option<String>,
    pub file_name: String,
    pub mime_type: String,
    pub byte_size: i64,
    pub sort_order: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSubscriptionCommand {
    pub target_type: String,
    pub target_id: i64,
    pub notify_level: String,
    pub delivery_channels: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSubscriptionCommand {
    pub subscription_id: i64,
    pub notify_level: Option<String>,
    pub delivery_channels: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListSubscriptionsCommand {
    pub target_type: Option<String>,
    pub target_id: Option<i64>,
    pub cursor: Option<String>,
    pub limit: u16,
}
