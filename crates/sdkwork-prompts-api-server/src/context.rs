use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, StatusCode},
};
use sdkwork_intelligence_prompts_service::value_objects::PromptsRequestContext;

use crate::auth::parse_access_token_header;

#[derive(Clone, Debug)]
pub struct ResolvedPromptsContext(pub PromptsRequestContext);

pub struct PromptsCtx(pub PromptsRequestContext);

#[async_trait]
impl<S> FromRequestParts<S> for PromptsCtx
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(resolved) = parts.extensions.get::<ResolvedPromptsContext>() {
            let mut ctx = resolved.0.clone();
            if let Some(request_id) = header_string(&parts.headers, "x-request-id") {
                ctx = ctx.with_request_id(request_id);
            }
            return Ok(PromptsCtx(ctx));
        }

        Ok(PromptsCtx(build_context(&parts.headers)))
    }
}

pub fn build_context(headers: &HeaderMap) -> PromptsRequestContext {
    if let Some(claims) = parse_access_token_header(headers) {
        let mut ctx = PromptsRequestContext::new(
            claims.tenant_id,
            claims.organization_id,
            claims.user_id,
        );
        if let Some(request_id) = header_string(headers, "x-request-id") {
            ctx = ctx.with_request_id(request_id);
        }
        return ctx;
    }

    let tenant_id = header_i64(headers, "x-sdkwork-tenant-id")
        .or_else(|| env_i64("SDKWORK_PROMPTS_DEFAULT_TENANT_ID"))
        .unwrap_or(1);
    let organization_id = header_i64(headers, "x-sdkwork-organization-id")
        .or_else(|| env_i64("SDKWORK_PROMPTS_DEFAULT_ORGANIZATION_ID"))
        .unwrap_or(0);
    let user_id = header_i64(headers, "x-sdkwork-user-id")
        .or_else(|| env_i64("SDKWORK_PROMPTS_DEFAULT_USER_ID"))
        .unwrap_or(1);

    let mut ctx = PromptsRequestContext::new(tenant_id, organization_id, user_id);
    if let Some(request_id) = header_string(headers, "x-request-id") {
        ctx = ctx.with_request_id(request_id);
    }
    ctx
}

fn header_i64(headers: &HeaderMap, name: &str) -> Option<i64> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())
}

fn header_string(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

fn env_i64(name: &str) -> Option<i64> {
    std::env::var(name).ok()?.parse().ok()
}

pub fn page_json<T: serde::Serialize>(
    page: &sdkwork_intelligence_prompts_service::domain::results::CursorPage<T>,
) -> serde_json::Value {
    serde_json::json!({
        "items": page.items,
        "nextCursor": page.next_cursor,
        "hasMore": page.has_more
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn build_context_prefers_access_token_header() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Access-Token",
            HeaderValue::from_static("tenant_id=42;organization_id=7;user_id=99"),
        );

        let ctx = build_context(&headers);
        assert_eq!(ctx.tenant_id_value(), 42);
        assert_eq!(ctx.organization_id_value(), 7);
        assert_eq!(ctx.user_id_value(), 99);
    }
}
