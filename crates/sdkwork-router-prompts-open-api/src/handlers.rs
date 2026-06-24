use crate::error::PromptsRouteError;

pub type HandlerResult = Result<Vec<u8>, PromptsRouteError>;

pub fn handle_boards_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("boards.list"))
}

pub fn handle_boards_topics_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("boards.topics.list"))
}

pub fn handle_topics_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.list"))
}

pub fn handle_topics_retrieve(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.retrieve"))
}

pub fn handle_topics_by_slug_retrieve(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.bySlug.retrieve"))
}

pub fn handle_topics_replies_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("topics.replies.list"))
}

pub fn handle_tags_list(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("tags.list"))
}

pub fn handle_search_query(_path: &str, _query: &str) -> HandlerResult {
    Err(PromptsRouteError::not_implemented("search.query"))
}
