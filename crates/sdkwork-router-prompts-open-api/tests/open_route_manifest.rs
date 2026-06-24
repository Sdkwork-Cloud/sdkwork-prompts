use sdkwork_router_prompts_open_api::{open_route_manifest, routes::OPEN_ROUTES};
use sdkwork_web_core::RouteAuth;

#[test]
fn open_route_manifest_declares_public_auth_for_all_operations() {
    let manifest = open_route_manifest();
    assert_eq!(OPEN_ROUTES.len(), 8);
    for entry in OPEN_ROUTES {
        let matched = manifest
            .match_route(entry.method, entry.path)
            .unwrap_or_else(|| {
                panic!(
                    "missing http route manifest for {} {}",
                    entry.method, entry.path
                )
            });
        assert_eq!(matched.auth, RouteAuth::Public);
        assert_eq!(matched.operation_id, entry.operation_id);
    }
}
