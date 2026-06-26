use axum::Router;
use sdkwork_web_bootstrap::{mount_infra_routes, ServiceRouterConfig};

/// Mounts standard infra routes on a stateful router (prompts uses `Router<AppState>`).
pub fn mount_service_routes<S>(router: Router<S>, config: ServiceRouterConfig) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    mount_infra_routes(router, config)
}
