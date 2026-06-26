use crate::error::PromptsRouteError;

pub type HandlerResult = Result<Vec<u8>, PromptsRouteError>;

pub fn handle_nodes_tree_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("nodes.tree.list"))
}

pub fn handle_topics_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.list"))
}

pub fn handle_topics_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.create"))
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

pub fn handle_topics_replies_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.replies.list"))
}

pub fn handle_topics_replies_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.replies.create"))
}

pub fn handle_replies_update(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("replies.update"))
}

pub fn handle_replies_delete(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("replies.delete"))
}

pub fn handle_topics_revisions_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.revisions.list"))
}

pub fn handle_replies_revisions_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("replies.revisions.list"))
}

pub fn handle_questions_accepted_reply_update(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("questions.acceptedReply.update"))
}

pub fn handle_questions_accepted_reply_delete(_path: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("questions.acceptedReply.delete"))
}

pub fn handle_polls_votes_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("polls.votes.create"))
}

pub fn handle_reactions_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("reactions.create"))
}

pub fn handle_votes_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("votes.create"))
}

pub fn handle_bookmarks_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("bookmarks.create"))
}

pub fn handle_read_state_topics_update(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("readState.topics.update"))
}

pub fn handle_reports_create(_path: &str, _body: &[u8]) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("reports.create"))
}

pub fn handle_feed_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("feed.list"))
}

pub fn handle_search_query(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("search.query"))
}
