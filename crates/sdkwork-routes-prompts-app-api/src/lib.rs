pub mod error;
pub mod handlers;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;

pub use routes::{build_sdkwork_prm_app_api_router, RouteDescriptor};

pub fn gateway_mount() -> Vec<RouteDescriptor> {
    build_sdkwork_prm_app_api_router()
}
