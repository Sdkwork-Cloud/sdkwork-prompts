use axum::{body::Body, http::Request, middleware::Next, response::Response};
use sdkwork_web_core::{
    problem_response, ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind,
};

use crate::response::anonymous_trace_id;

pub async fn require_dual_token_auth(request: Request<Body>, next: Next) -> Response {
    if !auth_required() {
        return next.run(request).await;
    }

    let path = request.uri().path();
    if !requires_auth(path) {
        return next.run(request).await;
    }

    let headers = request.headers();
    let has_auth = headers.contains_key("authorization");
    let has_access = headers.contains_key("access-token") || headers.contains_key("Access-Token");

    if has_auth && has_access {
        next.run(request).await
    } else {
        let trace_id = anonymous_trace_id();
        problem_response(
            &WebFrameworkError {
                kind: WebFrameworkErrorKind::MissingCredentials,
                message: "Authorization and Access-Token headers are required".to_string(),
                retry_after_seconds: None,
            },
            ProblemCorrelation::new(None, Some(&trace_id)),
        )
    }
}

fn auth_required() -> bool {
    matches!(
        std::env::var("SDKWORK_PROMPTS_REQUIRE_AUTH").as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE")
    )
}

fn requires_auth(path: &str) -> bool {
    path.starts_with("/app/v3/api/prompts") || path.starts_with("/backend/v3/api/prompts")
}
