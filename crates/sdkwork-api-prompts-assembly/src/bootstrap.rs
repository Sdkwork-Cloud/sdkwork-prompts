//! API assembly bootstrap for sdkwork-prompts.

use axum::middleware::{from_fn, from_fn_with_state};
use axum::Router;
use sdkwork_database_ops_http::{attach_ops_routes, default_ops_auth, DatabaseOpsHttpState};
use sdkwork_prompts_service_host::{default_seed_locale, default_seed_profile, PromptsServiceHost};
use sdkwork_prompts_web_context::{require_dual_token_auth, resolve_iam_context, AppState};

pub struct ApiAssembly {
    pub router: Router,
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    let service_host = PromptsServiceHost::try_new().await?;
    let state = AppState::new(
        service_host.ai_repository(),
        service_host.iam_pool().cloned(),
    );

    let business_router = Router::new()
        .merge(sdkwork_routes_prompts_app_api::routes())
        .merge(sdkwork_routes_prompts_backend_api::routes())
        .merge(sdkwork_routes_prompts_open_api::routes())
        .layer(from_fn(require_dual_token_auth))
        .layer(from_fn_with_state(state.clone(), resolve_iam_context))
        .with_state(state);

    let ops_state = DatabaseOpsHttpState::new(
        service_host.database_pool(),
        service_host.database_module(),
        default_seed_locale(),
        default_seed_profile(),
        default_ops_auth(),
    );
    let router = attach_ops_routes(business_router, ops_state).layer(
        sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_PROMPTS_ENVIRONMENT"],
            &[
                "SDKWORK_PROMPTS_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        ),
    );

    Ok(ApiAssembly { router })
}
