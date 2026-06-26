use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Deserialize)]
struct RouteManifest {
    routes: Vec<RouteManifestEntry>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct RouteManifestEntry {
    method: String,
    path: String,
    #[serde(rename = "operationId")]
    operation_id: String,
    auth: RouteManifestAuth,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct RouteManifestAuth {
    mode: String,
}

#[derive(Debug, Clone)]
pub struct PromptsRouteInfo {
    pub surface: String,
    pub method: String,
    pub path: String,
    pub operation_id: String,
    pub auth_mode: String,
}

fn manifest_paths() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "app-api",
            "sdks/_route-manifests/app-api/sdkwork-routes-prompts-app-api.route-manifest.json",
        ),
        (
            "backend-api",
            "sdks/_route-manifests/backend-api/sdkwork-routes-prompts-backend-api.route-manifest.json",
        ),
        (
            "open-api",
            "sdks/_route-manifests/open-api/sdkwork-routes-prompts-open-api.route-manifest.json",
        ),
    ]
}

fn app_root() -> PathBuf {
    std::env::var("SDKWORK_PROMPTS_APP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        })
}

pub fn compose_prompts_api_routes() -> Vec<PromptsRouteInfo> {
    let root = app_root();
    let mut routes = Vec::new();

    for (surface, relative_path) in manifest_paths() {
        let path = root.join(relative_path);
        let text = fs::read_to_string(&path).unwrap_or_else(|error| {
            panic!("failed to read route manifest {}: {error}", path.display())
        });
        let manifest: RouteManifest =
            serde_json::from_str(&text).unwrap_or_else(|error| {
                panic!("failed to parse route manifest {}: {error}", path.display())
            });

        for route in manifest.routes {
            routes.push(PromptsRouteInfo {
                surface: surface.to_string(),
                method: route.method,
                path: route.path,
                operation_id: route.operation_id,
                auth_mode: route.auth.mode,
            });
        }
    }

    routes
}

pub fn route_count() -> usize {
    compose_prompts_api_routes().len()
}

pub fn route_count_by_surface(surface: &str) -> usize {
    compose_prompts_api_routes()
        .iter()
        .filter(|route| route.surface == surface)
        .count()
}

fn path_matches(template: &str, actual: &str) -> bool {
    let template_segments: Vec<&str> = template.split('/').collect();
    let actual_segments: Vec<&str> = actual.split('/').collect();
    if template_segments.len() != actual_segments.len() {
        return false;
    }
    template_segments
        .iter()
        .zip(actual_segments.iter())
        .all(|(template_segment, actual_segment)| {
            template_segment.starts_with('{') || template_segment == actual_segment
        })
}

pub fn find_route(method: &str, path: &str) -> Option<PromptsRouteInfo> {
    compose_prompts_api_routes()
        .into_iter()
        .find(|route| route.method == method && path_matches(&route.path, path))
}
