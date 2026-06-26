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
        Self { method, path, operation_id, surface: "backend-api", auth_mode, tags }
    }
}

pub const BACKEND_ROUTES: &[RouteDescriptor] = &[
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/nodes", "nodes.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/nodes", "nodes.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("PATCH", "/backend/v3/api/prompts/nodes/{nodeId}", "nodes.update", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/prompts/nodes/{nodeId}", "nodes.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/topic_prefixes", "topicPrefixes.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/topic_prefixes", "topicPrefixes.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/topics", "topics.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/topics/{topicId}", "topics.retrieve", "dual-token", &["intelligence"]),
    RouteDescriptor::new("PATCH", "/backend/v3/api/prompts/topics/{topicId}", "topics.update", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/prompts/topics/{topicId}", "topics.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/topics/{topicId}/pin", "topics.pin.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/prompts/topics/{topicId}/pin", "topics.pin.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/topics/{topicId}/feature", "topics.feature.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/prompts/topics/{topicId}/feature", "topics.feature.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/topics/{topicId}/lock", "topics.lock.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("DELETE", "/backend/v3/api/prompts/topics/{topicId}/lock", "topics.lock.delete", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/topics/{topicId}/move", "topics.move.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/moderation/queue", "moderation.queue.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/moderation/cases", "moderation.cases.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/moderation/cases", "moderation.cases.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/moderation/cases/{caseId}", "moderation.cases.retrieve", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/moderation/cases/{caseId}/decisions", "moderation.cases.decisions.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/sanctions", "sanctions.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/sanctions", "sanctions.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("PATCH", "/backend/v3/api/prompts/sanctions/{sanctionId}", "sanctions.update", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/reputation/rules", "reputation.rules.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/reputation/rules", "reputation.rules.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/reputation/ledger", "reputation.ledger.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/trust_levels", "trustLevels.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/trust_levels", "trustLevels.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/badges", "badges.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/badges", "badges.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/stats/boards", "stats.boards.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/stats/topics", "stats.topics.list", "dual-token", &["intelligence"]),
    RouteDescriptor::new("POST", "/backend/v3/api/prompts/search/reindex", "search.reindex.create", "dual-token", &["intelligence"]),
    RouteDescriptor::new("GET", "/backend/v3/api/prompts/audit/actions", "audit.actions.list", "dual-token", &["intelligence"]),
];

pub fn build_sdkwork_prm_backend_api_router() -> Vec<RouteDescriptor> {
    BACKEND_ROUTES.to_vec()
}

pub fn find_route(method: &str, path: &str) -> Option<&'static RouteDescriptor> {
    BACKEND_ROUTES.iter().find(|r| r.method == method && path_matches(r.path, path))
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
