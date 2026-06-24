use super::models::*;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPage<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

impl<T> CursorPage<T> {
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            next_cursor: None,
            has_more: false,
        }
    }

    pub fn new(items: Vec<T>, next_cursor: Option<String>, has_more: bool) -> Self {
        Self { items, next_cursor, has_more }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub id: Option<i64>,
    pub uuid: Option<String>,
    pub status: Option<String>,
}

impl CommandResult {
    pub fn success(id: i64, uuid: impl Into<String>) -> Self {
        Self {
            success: true,
            id: Some(id),
            uuid: Some(uuid.into()),
            status: None,
        }
    }

    pub fn success_with_status(id: i64, uuid: impl Into<String>, status: impl Into<String>) -> Self {
        Self {
            success: true,
            id: Some(id),
            uuid: Some(uuid.into()),
            status: Some(status.into()),
        }
    }

    pub fn no_id() -> Self {
        Self {
            success: true,
            id: None,
            uuid: None,
            status: None,
        }
    }
}

pub type NodeTreeResult = Vec<PromptsNode>;
pub type TopicPageResult = CursorPage<PromptsTopic>;
pub type ReplyPageResult = CursorPage<PromptsReply>;
pub type FeedPageResult = CursorPage<PromptsFeedItem>;
pub type SearchResult = CursorPage<PromptsSearchDocument>;
pub type ModerationQueueResult = CursorPage<PromptsModerationCase>;
pub type ModerationDecisionResult = PromptsModerationDecision;
pub type TopicRevisionPageResult = CursorPage<PromptsTopicRevision>;
pub type ReplyRevisionPageResult = CursorPage<PromptsReplyRevision>;
pub type ModerationCasePageResult = CursorPage<PromptsModerationCase>;
pub type SanctionPageResult = CursorPage<PromptsSanction>;
pub type ReputationRulePageResult = CursorPage<PromptsReputationRule>;
pub type ReputationLedgerPageResult = CursorPage<PromptsReputationLedger>;
pub type TrustLevelPageResult = CursorPage<PromptsTrustLevel>;
pub type BadgePageResult = CursorPage<PromptsBadge>;
pub type BoardStatsPageResult = CursorPage<PromptsBoardStats>;
pub type TopicStatsPageResult = CursorPage<PromptsTopicStats>;
pub type TopicPrefixPageResult = CursorPage<PromptsTopicPrefix>;
pub type AuditActionPageResult = CursorPage<PromptsAuditAction>;
pub type NodePageResult = CursorPage<PromptsNode>;
pub type TagPageResult = CursorPage<PromptsTag>;
pub type SubscriptionPageResult = CursorPage<PromptsSubscription>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptsReputationRule {
    pub id: i64,
    pub uuid: String,
    pub code: String,
    pub event_type: String,
    pub points: i64,
    pub daily_limit: Option<i64>,
    pub rule_json: Option<serde_json::Value>,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptsReputationLedger {
    pub id: i64,
    pub uuid: String,
    pub user_id: i64,
    pub source_type: String,
    pub source_id: Option<i64>,
    pub direction: String,
    pub points: i64,
    pub balance_after: i64,
    pub reason_code: String,
    pub idempotency_key: String,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptsTrustLevel {
    pub id: i64,
    pub uuid: String,
    pub level_no: i32,
    pub code: String,
    pub name: String,
    pub threshold_rules: Option<serde_json::Value>,
    pub privileges: Option<serde_json::Value>,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptsBadge {
    pub id: i64,
    pub uuid: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub grant_mode: String,
    pub icon_media_id: Option<String>,
    pub rule_json: Option<serde_json::Value>,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptsTopicPrefix {
    pub id: i64,
    pub uuid: String,
    pub board_id: i64,
    pub code: String,
    pub label: String,
    pub color: Option<String>,
    pub sort_order: i32,
    pub required_trust_level: Option<i32>,
    pub status: String,
    pub version: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptsAuditAction {
    pub id: i64,
    pub uuid: String,
    pub action: String,
    pub target_type: String,
    pub target_id: i64,
    pub operator_id: i64,
    pub detail: Option<String>,
    pub request_id: Option<String>,
    pub created_at: String,
}
