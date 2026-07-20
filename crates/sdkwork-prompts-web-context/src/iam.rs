use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use sdkwork_iam_web_adapter::resolve_iam_app_context_from_dual_tokens;

use crate::context::{PromptsRequestContext, ResolvedPromptsContext};
use crate::AppState;

pub fn iam_enabled() -> bool {
    matches!(
        std::env::var("SDKWORK_PROMPTS_IAM_ENABLED").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

fn iam_strict() -> bool {
    matches!(
        std::env::var("SDKWORK_PROMPTS_IAM_STRICT").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

pub async fn resolve_iam_context(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    if !iam_enabled() {
        return next.run(request).await;
    }

    let path = request.uri().path().to_owned();
    if !requires_iam_resolution(&path) {
        return next.run(request).await;
    }

    let Some(pool) = state.iam_pool() else {
        return next.run(request).await;
    };

    let headers = request.headers().clone();
    let auth = header_value(&headers, "authorization");
    let access =
        header_value(&headers, "access-token").or_else(|| header_value(&headers, "Access-Token"));

    match (auth, access) {
        (Some(auth), Some(access)) => {
            match resolve_iam_app_context_from_dual_tokens(pool, &auth, &access).await {
                Some(iam) => {
                    request
                        .extensions_mut()
                        .insert(ResolvedPromptsContext(prompts_context_from_iam(&iam)));
                }
                None if iam_strict() => return unauthorized("invalid or expired IAM session"),
                None => {}
            }
        }
        _ if iam_strict() && requires_protected_surface(&path) => {
            return unauthorized("Authorization and Access-Token headers are required");
        }
        _ => {}
    }

    next.run(request).await
}

fn prompts_context_from_iam(
    iam: &sdkwork_iam_context_service::IamAppContext,
) -> PromptsRequestContext {
    let tenant_id = iam.tenant_id.parse().unwrap_or(0);
    let organization_id = iam
        .organization_id
        .as_deref()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);
    let user_id = iam.user_id.parse().unwrap_or(0);
    PromptsRequestContext::new(tenant_id, organization_id, user_id)
}

fn requires_iam_resolution(path: &str) -> bool {
    if path.starts_with("/prompts/v3/api") {
        return false;
    }
    path.starts_with("/app/v3/api/prompts") || path.starts_with("/backend/v3/api/prompts")
}

fn requires_protected_surface(path: &str) -> bool {
    requires_iam_resolution(path)
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

use crate::response::anonymous_trace_id;
use sdkwork_web_core::{
    problem_response, ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind,
};

fn unauthorized(message: &str) -> Response {
    let trace_id = anonymous_trace_id();
    problem_response(
        &WebFrameworkError {
            kind: WebFrameworkErrorKind::MissingCredentials,
            message: message.to_string(),
            retry_after_seconds: None,
        },
        ProblemCorrelation::new(None, Some(&trace_id)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompts_context_from_iam_parses_numeric_ids() {
        let iam = sdkwork_iam_context_service::IamAppContext::new(
            "42",
            Some("7"),
            "99",
            "session-1",
            "prompts",
            sdkwork_iam_context_service::Environment::Dev,
            sdkwork_iam_context_service::DeploymentMode::Local,
            sdkwork_iam_context_service::AuthLevel::Password,
            Vec::new(),
            Vec::new(),
        );
        let ctx = prompts_context_from_iam(&iam);
        assert_eq!(ctx.tenant_id_value(), 42);
        assert_eq!(ctx.organization_id_value(), 7);
        assert_eq!(ctx.user_id_value(), 99);
    }
}
