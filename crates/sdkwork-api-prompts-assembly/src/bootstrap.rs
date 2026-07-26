//! API assembly bootstrap for sdkwork-prompts.

use std::collections::BTreeSet;
use std::sync::Arc;

use axum::Router;
use sdkwork_database_ops_http::{attach_ops_routes, default_ops_auth, DatabaseOpsHttpState};
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_prompts_service_host::{default_seed_locale, default_seed_profile, PromptsServiceHost};
use sdkwork_prompts_web_context::{AppState, PromptsRequestContext, ResolvedPromptsContext};
use sdkwork_web_axum::{with_web_request_context, WebFrameworkLayer};
use sdkwork_web_bootstrap::{ReadinessCheck, ReadinessFuture};
use sdkwork_web_core::{
    DomainContextInjector, HttpRoute, HttpRouteManifest, WebRequestContext,
    WebRequestContextProfile,
};

pub struct ApiAssembly {
    pub router: Router,
}

pub struct ApiAssemblyContribution {
    pub router: Router,
    pub route_manifest: HttpRouteManifest,
    pub openapi: serde_json::Value,
    pub permission_catalog: Vec<&'static str>,
    pub domain_context_injectors: Vec<Arc<dyn DomainContextInjector>>,
    pub readiness_check: Arc<dyn ReadinessCheck>,
}

#[derive(Clone)]
struct PromptsReadiness {
    pool: DatabasePool,
}

impl ReadinessCheck for PromptsReadiness {
    fn check(&self) -> ReadinessFuture<'_> {
        let pool = self.pool.clone();
        Box::pin(async move {
            match pool.test_connection().await {
                Ok(true) => Ok(()),
                Ok(false) => Err("prompts database readiness query returned no row".to_owned()),
                Err(error) => Err(format!("prompts database readiness check failed: {error}")),
            }
        })
    }
}

#[derive(Clone, Default)]
struct PromptsContextInjector;

impl DomainContextInjector for PromptsContextInjector {
    fn inject(&self, request: &mut axum::extract::Request, context: &WebRequestContext) {
        let Some(principal) = context.principal.as_ref() else {
            return;
        };
        let (Ok(tenant_id), Ok(user_id)) =
            (principal.tenant_id().parse(), principal.user_id().parse())
        else {
            return;
        };
        let organization_id = principal
            .organization_id()
            .and_then(|value| value.parse().ok())
            .unwrap_or(0);
        request
            .extensions_mut()
            .insert(ResolvedPromptsContext(PromptsRequestContext::new(
                tenant_id,
                organization_id,
                user_id,
            )));
    }
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    let service_host = PromptsServiceHost::try_new().await?;
    let state = AppState::new(
        service_host.ai_repository(),
        service_host.iam_pool().cloned(),
    );

    let business_routes = Router::new()
        .merge(sdkwork_routes_prompts_app_api::routes())
        .merge(sdkwork_routes_prompts_backend_api::routes())
        .merge(sdkwork_routes_prompts_open_api::routes())
        .with_state(state);
    let routes = sdkwork_routes_prompts_app_api::app_route_manifest()
        .routes()
        .iter()
        .chain(sdkwork_routes_prompts_backend_api::backend_route_manifest().routes())
        .chain(sdkwork_routes_prompts_open_api::open_route_manifest().routes())
        .cloned()
        .collect();
    let route_manifest = HttpRouteManifest::from_owned_routes(routes);
    let resolver = sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env().await;
    let layer = WebFrameworkLayer::new(resolver)
        .with_profile(WebRequestContextProfile {
            open_api_prefixes: vec!["/prompts/v3/api".to_owned()],
            ..WebRequestContextProfile::default()
        })
        .with_route_manifest(route_manifest)
        .with_domain_injector(Arc::new(PromptsContextInjector));
    let business_router = with_web_request_context(business_routes, layer);

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

/// Builds the raw Prompts App API for a gateway-owned Web Framework layer.
pub async fn assemble_app_api_contribution() -> Result<ApiAssemblyContribution, String> {
    let service_host = PromptsServiceHost::try_new().await?;
    let readiness_check = Arc::new(PromptsReadiness {
        pool: service_host.database_pool(),
    });
    let state = AppState::new(
        service_host.ai_repository(),
        service_host.iam_pool().cloned(),
    );
    let route_manifest = sdkwork_routes_prompts_app_api::app_route_manifest();
    let router = sdkwork_routes_prompts_app_api::gateway_mount(state);
    Ok(ApiAssemblyContribution {
        router,
        openapi: sdkwork_web_contract::build_openapi_document(
            "SDKWork Prompts App API",
            route_manifest.routes(),
        ),
        permission_catalog: permission_catalog(route_manifest.routes()),
        route_manifest,
        domain_context_injectors: vec![Arc::new(PromptsContextInjector)],
        readiness_check,
    })
}

fn permission_catalog(routes: &[HttpRoute]) -> Vec<&'static str> {
    let mut permissions = BTreeSet::new();
    for route in routes {
        if let Some(permission) = route.required_permission {
            permissions.insert(permission);
        }
        if let Some(alternate_permissions) = route.alternate_permissions {
            permissions.extend(alternate_permissions.iter().copied());
        }
    }
    permissions.into_iter().collect()
}
