use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/boards",
        "intelligence",
        "boards.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/boards/{boardId}/topics",
        "intelligence",
        "boards.topics.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/topics",
        "intelligence",
        "topics.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/topics/{topicId}",
        "intelligence",
        "topics.retrieve",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/topics/by_slug/{topicSlug}",
        "intelligence",
        "topics.bySlug.retrieve",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/topics/{topicId}/replies",
        "intelligence",
        "topics.replies.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/tags",
        "intelligence",
        "tags.list",
    ),
    HttpRoute::public(
        HttpMethod::Get,
        "/prompts/v3/api/sites/{siteSlug}/search",
        "intelligence",
        "search.query",
    ),
];

pub fn open_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
