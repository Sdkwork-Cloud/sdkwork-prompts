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
        Self { method, path, operation_id, surface: "open-api", auth_mode, tags }
    }
}

pub const OPEN_ROUTES: &[RouteDescriptor] = &[
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/boards", "boards.list", "public", &["intelligence"]),
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/boards/{boardId}/topics", "boards.topics.list", "public", &["intelligence"]),
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/topics", "topics.list", "public", &["intelligence"]),
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/topics/{topicId}", "topics.retrieve", "public", &["intelligence"]),
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/topics/by_slug/{topicSlug}", "topics.bySlug.retrieve", "public", &["intelligence"]),
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/topics/{topicId}/replies", "topics.replies.list", "public", &["intelligence"]),
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/tags", "tags.list", "public", &["intelligence"]),
    RouteDescriptor::new("GET", "/prompts/v3/api/sites/{siteSlug}/search", "search.query", "public", &["intelligence"]),
];

pub fn build_sdkwork_prm_open_api_router() -> Vec<RouteDescriptor> {
    OPEN_ROUTES.to_vec()
}

pub fn find_route(method: &str, path: &str) -> Option<&'static RouteDescriptor> {
    OPEN_ROUTES.iter().find(|r| r.method == method && path_matches(r.path, path))
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
