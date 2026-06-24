pub const PREFIX: &str = "/prompts/v3/api";

pub const ROUTES: &[(&str, &str, &str)] = &[
    ("GET", "/prompts/v3/api/sites/{siteSlug}/boards", "boards.list"),
    ("GET", "/prompts/v3/api/sites/{siteSlug}/boards/{boardId}/topics", "boards.topics.list"),
    ("GET", "/prompts/v3/api/sites/{siteSlug}/topics", "topics.list"),
    ("GET", "/prompts/v3/api/sites/{siteSlug}/topics/{topicId}", "topics.retrieve"),
    ("GET", "/prompts/v3/api/sites/{siteSlug}/topics/by_slug/{topicSlug}", "topics.bySlug.retrieve"),
    ("GET", "/prompts/v3/api/sites/{siteSlug}/topics/{topicId}/replies", "topics.replies.list"),
    ("GET", "/prompts/v3/api/sites/{siteSlug}/tags", "tags.list"),
    ("GET", "/prompts/v3/api/sites/{siteSlug}/search", "search.query"),
];
