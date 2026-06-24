pub mod error;
pub mod handlers;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;

pub use routes::{build_sdkwork_prm_backend_api_router, RouteDescriptor};
