use crate::error::PromptsRouteError;

pub type HandlerResult = Result<Vec<u8>, PromptsRouteError>;

pub fn handle_nodes_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("nodes.list"))
}

pub fn handle_nodes_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("nodes.create"))
}

pub fn handle_nodes_update(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("nodes.update"))
}

pub fn handle_nodes_delete(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("nodes.delete"))
}

pub fn handle_topic_prefixes_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topicPrefixes.list"))
}

pub fn handle_topic_prefixes_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topicPrefixes.create"))
}

pub fn handle_topics_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.list"))
}

pub fn handle_topics_retrieve(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.retrieve"))
}

pub fn handle_topics_update(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.update"))
}

pub fn handle_topics_delete(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.delete"))
}

pub fn handle_topics_pin_create(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.pin.create"))
}

pub fn handle_topics_pin_delete(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.pin.delete"))
}

pub fn handle_topics_feature_create(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.feature.create"))
}

pub fn handle_topics_feature_delete(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.feature.delete"))
}

pub fn handle_topics_lock_create(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.lock.create"))
}

pub fn handle_topics_lock_delete(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.lock.delete"))
}

pub fn handle_topics_move_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.move.create"))
}

pub fn handle_moderation_queue_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("moderation.queue.list"))
}

pub fn handle_moderation_cases_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("moderation.cases.list"))
}

pub fn handle_moderation_cases_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("moderation.cases.create"))
}

pub fn handle_moderation_cases_retrieve(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("moderation.cases.retrieve"))
}

pub fn handle_moderation_cases_decisions_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("moderation.cases.decisions.create"))
}

pub fn handle_sanctions_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("sanctions.list"))
}

pub fn handle_sanctions_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("sanctions.create"))
}

pub fn handle_sanctions_update(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("sanctions.update"))
}

pub fn handle_reputation_rules_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("reputation.rules.list"))
}

pub fn handle_reputation_rules_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("reputation.rules.create"))
}

pub fn handle_reputation_ledger_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("reputation.ledger.list"))
}

pub fn handle_trust_levels_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("trustLevels.list"))
}

pub fn handle_trust_levels_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("trustLevels.create"))
}

pub fn handle_badges_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("badges.list"))
}

pub fn handle_badges_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("badges.create"))
}

pub fn handle_stats_boards_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("stats.boards.list"))
}

pub fn handle_stats_topics_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("stats.topics.list"))
}

pub fn handle_search_reindex_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("search.reindex.create"))
}

pub fn handle_audit_actions_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("audit.actions.list"))
}
