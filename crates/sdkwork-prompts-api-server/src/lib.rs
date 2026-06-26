pub fn compose_prm_api_routes() -> Vec<PromptsRouteInfo> {
    let app_routes = sdkwork_routes_prompts_app_api::build_sdkwork_prm_app_api_router();
    let backend_routes = sdkwork_routes_prompts_backend_api::build_sdkwork_prm_backend_api_router();
    let open_routes = sdkwork_routes_prompts_open_api::build_sdkwork_prm_open_api_router();

    let mut all_routes: Vec<PromptsRouteInfo> = Vec::new();

    for route in &app_routes {
        all_routes.push(PromptsRouteInfo {
            surface: route.surface.to_string(),
            method: route.method.to_string(),
            path: route.path.to_string(),
            operation_id: route.operation_id.to_string(),
            auth_mode: route.auth_mode.to_string(),
        });
    }

    for route in &backend_routes {
        all_routes.push(PromptsRouteInfo {
            surface: route.surface.to_string(),
            method: route.method.to_string(),
            path: route.path.to_string(),
            operation_id: route.operation_id.to_string(),
            auth_mode: route.auth_mode.to_string(),
        });
    }

    for route in &open_routes {
        all_routes.push(PromptsRouteInfo {
            surface: route.surface.to_string(),
            method: route.method.to_string(),
            path: route.path.to_string(),
            operation_id: route.operation_id.to_string(),
            auth_mode: route.auth_mode.to_string(),
        });
    }

    all_routes
}

#[derive(Debug, Clone)]
pub struct PromptsRouteInfo {
    pub surface: String,
    pub method: String,
    pub path: String,
    pub operation_id: String,
    pub auth_mode: String,
}

pub fn route_count() -> usize {
    compose_prm_api_routes().len()
}

pub fn route_count_by_surface(surface: &str) -> usize {
    compose_prm_api_routes().iter().filter(|r| r.surface == surface).count()
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

pub fn find_route(method: &str, path: &str) -> Option<PromptsRouteInfo> {
    compose_prm_api_routes().into_iter().find(|r| r.method == method && path_matches(&r.path, path))
}
