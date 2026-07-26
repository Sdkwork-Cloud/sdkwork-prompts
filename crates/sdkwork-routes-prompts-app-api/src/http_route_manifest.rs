use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/prompts/templates",
        "prompts",
        "prompts.templates.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/prompts/templates",
        "prompts",
        "prompts.templates.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/prompts/templates/{templateId}",
        "prompts",
        "prompts.templates.retrieve",
    ),
    HttpRoute::dual_token(
        HttpMethod::Patch,
        "/app/v3/api/prompts/templates/{templateId}",
        "prompts",
        "prompts.templates.update",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/prompts/templates/{templateId}/versions",
        "prompts",
        "prompts.templateVersions.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Post,
        "/app/v3/api/prompts/templates/{templateId}/versions",
        "prompts",
        "prompts.templateVersions.create",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/prompts/agent_templates",
        "prompts",
        "prompts.agentTemplates.list",
    ),
    HttpRoute::dual_token(
        HttpMethod::Get,
        "/app/v3/api/prompts/agent_templates/{templateId}",
        "prompts",
        "prompts.agentTemplates.retrieve",
    ),
];

pub fn app_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
