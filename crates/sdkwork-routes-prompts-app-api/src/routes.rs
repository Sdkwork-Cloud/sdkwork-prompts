#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDescriptor {
    pub method: &'static str,
    pub path: &'static str,
    pub operation_id: &'static str,
    pub surface: &'static str,
    pub auth_mode: &'static str,
    pub tags: &'static [&'static str],
}

impl RouteDescriptor {
    pub const fn new(
        method: &'static str,
        path: &'static str,
        operation_id: &'static str,
        auth_mode: &'static str,
        tags: &'static [&'static str],
    ) -> Self {
        Self { method, path, operation_id, surface: "app-api", auth_mode, tags }
    }
}

pub const APP_ROUTES: &[RouteDescriptor] = &[
    RouteDescriptor::new("GET", "/app/v3/api/prompts/nodes/tree", "nodes.tree.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/app/v3/api/prompts/boards/{boardId}/topics", "topics.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/app/v3/api/prompts/topics", "topics.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/app/v3/api/prompts/topics/{topicId}", "topics.retrieve", "dual-token", &["intelligence"]),
    RouteDescriptor::new("PATCH", "/app/v3/api/prompts/topics/{topicId}", "topics.update", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/app/v3/api/prompts/topics/{topicId}", "topics.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/app/v3/api/prompts/topics/{topicId}/replies", "topics.replies.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/app/v3/api/prompts/topics/{topicId}/replies", "topics.replies.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("PATCH", "/app/v3/api/prompts/replies/{replyId}", "replies.update", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/app/v3/api/prompts/replies/{replyId}", "replies.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/app/v3/api/prompts/topics/{topicId}/revisions", "topics.revisions.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/app/v3/api/prompts/replies/{replyId}/revisions", "replies.revisions.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("PUT", "/app/v3/api/prompts/questions/{topicId}/accepted_reply", "questions.acceptedReply.update", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/app/v3/api/prompts/questions/{topicId}/accepted_reply", "questions.acceptedReply.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/app/v3/api/prompts/polls/{pollId}/votes", "polls.votes.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/app/v3/api/prompts/reactions", "reactions.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/app/v3/api/prompts/votes", "votes.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/app/v3/api/prompts/bookmarks", "bookmarks.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("PATCH", "/app/v3/api/prompts/read_state/topics/{topicId}", "readState.topics.update", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/app/v3/api/prompts/reports", "reports.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/app/v3/api/prompts/feed", "feed.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/app/v3/api/prompts/search", "search.query", "dual-token", &["intelligence"]),
];

pub fn build_sdkwork_prm_app_api_router() -> Vec<RouteDescriptor> {
    APP_ROUTES.to_vec()
}

pub fn find_route(method: &str, path: &str) -> Option<&'static RouteDescriptor> {
    APP_ROUTES.iter().find(|r| r.method == method && path_matches(r.path, path))
}

fn path_matches(template: &str, actual: &str) -> bool {
    let template_segments: Vec<&str> = template.split('/').collect();
    let actual_segments: Vec<&str> = actual.split('/').collect();
    if template_segments.len() != actual_segments.len() {
        return false;
    }
    template_segments.iter().zip(actual_segments.iter()).all(|(t, a)| {
        t.starts_with('{') || t == a
    })
}
