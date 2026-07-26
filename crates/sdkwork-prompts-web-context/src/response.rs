use axum::{
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use sdkwork_intelligence_prompts_ai_contract::PromptAiError;
use sdkwork_utils_rust::{
    uuid, PageInfo, PageMode, SdkWorkApiResponse, SdkWorkPageData, SdkWorkResourceData,
};
use sdkwork_web_core::{
    problem_response, ProblemCorrelation, WebFrameworkError, WebFrameworkErrorKind,
};
use serde::Serialize;

use crate::context::PromptsRequestContext;

const SDKWORK_TRACE_ID_HEADER_LOWER: &str = "x-sdkwork-trace-id";

pub fn resolve_trace_id(ctx: &PromptsRequestContext) -> String {
    ctx.trace_id()
}

fn attach_trace_header(response: &mut Response, trace_id: &str) {
    if let Ok(value) = HeaderValue::from_str(trace_id) {
        response.headers_mut().insert(
            HeaderName::from_static(SDKWORK_TRACE_ID_HEADER_LOWER),
            value,
        );
    }
}

pub fn success_response<T: Serialize>(
    ctx: &PromptsRequestContext,
    status: StatusCode,
    data: T,
) -> Response {
    let trace_id = resolve_trace_id(ctx);
    let envelope = SdkWorkApiResponse::success(data, trace_id.clone());
    let mut response = (status, Json(envelope)).into_response();
    attach_trace_header(&mut response, &trace_id);
    response
}

pub fn ok_json<T: Serialize>(ctx: &PromptsRequestContext, data: T) -> Response {
    success_response(ctx, StatusCode::OK, data)
}

pub fn created_json<T: Serialize>(ctx: &PromptsRequestContext, data: T) -> Response {
    success_response(ctx, StatusCode::CREATED, data)
}

pub fn resource_data<T: Serialize>(item: T) -> SdkWorkResourceData<T> {
    SdkWorkResourceData { item }
}

pub fn page_data<T: Serialize>(items: Vec<T>, page_info: PageInfo) -> SdkWorkPageData<T> {
    SdkWorkPageData { items, page_info }
}

pub fn cursor_page_info(next_cursor: Option<String>, has_more: bool) -> PageInfo {
    PageInfo {
        mode: PageMode::Cursor,
        page: None,
        page_size: None,
        total_items: None,
        total_pages: None,
        next_cursor,
        has_more: Some(has_more),
    }
}

pub fn offset_page_info(page: i32, page_size: i32) -> PageInfo {
    PageInfo {
        mode: PageMode::Offset,
        page: Some(page),
        page_size: Some(page_size),
        total_items: None,
        total_pages: None,
        next_cursor: None,
        has_more: None,
    }
}

pub fn map_prompt_error(ctx: &PromptsRequestContext, error: PromptAiError) -> Response {
    let (kind, message) = match error {
        PromptAiError::NotFound(message) => (WebFrameworkErrorKind::NotFound, message),
        PromptAiError::Conflict(message) => (WebFrameworkErrorKind::Conflict, message),
        PromptAiError::Validation(message) => (WebFrameworkErrorKind::BadRequest, message),
        PromptAiError::Internal(message) => (WebFrameworkErrorKind::InternalServerError, message),
    };
    problem_for(ctx, kind, message)
}

pub fn status_problem(
    ctx: &PromptsRequestContext,
    kind: WebFrameworkErrorKind,
    message: impl Into<String>,
) -> Response {
    problem_for(ctx, kind, message.into())
}

fn problem_for(
    ctx: &PromptsRequestContext,
    kind: WebFrameworkErrorKind,
    message: String,
) -> Response {
    let trace_id = resolve_trace_id(ctx);
    problem_response(
        &WebFrameworkError {
            kind,
            message,
            retry_after_seconds: None,
            auth_profile: None,
            failed_stage: None,
            reason: None,
        },
        ProblemCorrelation::new(ctx.request_id().as_deref(), Some(&trace_id)),
    )
}

pub fn anonymous_trace_id() -> String {
    uuid()
}

pub fn anonymous_ok_json<T: Serialize>(data: T) -> Response {
    let trace_id = anonymous_trace_id();
    let envelope = SdkWorkApiResponse::success(data, trace_id.clone());
    let mut response = (StatusCode::OK, Json(envelope)).into_response();
    attach_trace_header(&mut response, &trace_id);
    response
}

pub fn anonymous_prompt_error(error: PromptAiError) -> Response {
    let trace_id = anonymous_trace_id();
    let (kind, message) = match error {
        PromptAiError::NotFound(message) => (WebFrameworkErrorKind::NotFound, message),
        PromptAiError::Conflict(message) => (WebFrameworkErrorKind::Conflict, message),
        PromptAiError::Validation(message) => (WebFrameworkErrorKind::BadRequest, message),
        PromptAiError::Internal(message) => (WebFrameworkErrorKind::InternalServerError, message),
    };
    problem_response(
        &WebFrameworkError {
            kind,
            message,
            retry_after_seconds: None,
            auth_profile: None,
            failed_stage: None,
            reason: None,
        },
        ProblemCorrelation::new(None, Some(&trace_id)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn success_response_attaches_lowercase_trace_header_without_panicking() {
        let ctx = PromptsRequestContext::new(100_001, 0, 1)
            .with_request_id("trace-prompts-success".to_string());

        let response = ok_json(&ctx, json!({ "status": "ok" }));

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(SDKWORK_TRACE_ID_HEADER_LOWER)
                .and_then(|value| value.to_str().ok()),
            Some("trace-prompts-success")
        );
    }

    #[test]
    fn anonymous_response_attaches_valid_trace_header_without_panicking() {
        let response = anonymous_ok_json(json!({ "status": "ok" }));

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response
            .headers()
            .get(SDKWORK_TRACE_ID_HEADER_LOWER)
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| !value.is_empty()));
    }
}
