use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[HttpRoute::public(
    HttpMethod::Get,
    "/prompts/v3/api/prompts/catalog",
    "prompts",
    "prompts.catalog.list",
)];

pub fn open_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
