use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use sdkwork_intelligence_prompts_ai_contract::{
    commands::{
        CreatePromptBindingCommand, CreatePromptCommand, CreatePromptVersionCommand,
        ListPromptBindingsQuery, ListPromptsQuery, ListPromptVersionsQuery, PromptAiSubject,
        PublishPromptVersionCommand, RenderPromptVersionCommand, UpdatePromptBindingCommand,
    },
    PromptAiError, PromptAiRepository,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::context::PromptsCtx;
use crate::dto::PlusApiResult;
use crate::AppState;

const DEFAULT_PAGE_NO: i64 = 1;
const DEFAULT_PAGE_SIZE: i64 = 50;
const MAX_PAGE_SIZE: i64 = 200;

#[derive(Debug, Default, Deserialize)]
struct ListPromptsRequest {
    page: Option<i64>,
    page_size: Option<i64>,
    q: Option<String>,
    prompt_type: Option<String>,
    visibility: Option<String>,
    status: Option<String>,
    category_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePromptRequest {
    prompt_key: String,
    name: String,
    description: Option<String>,
    category_id: Option<String>,
    prompt_type: Option<String>,
    visibility: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePromptVersionRequest {
    version_no: String,
    title: String,
    content: String,
    variable_schema: Option<Value>,
    output_schema: Option<Value>,
    model_constraints: Option<Value>,
    safety_policy: Option<Value>,
    examples_json: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RenderPromptVersionRequest {
    variables: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePromptBindingRequest {
    prompt_version_id: Option<i64>,
    owner_type: String,
    owner_id: i64,
    binding_role: String,
    priority: Option<i32>,
    enabled: Option<bool>,
    policy_json: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdatePromptBindingRequest {
    prompt_version_id: Option<Value>,
    owner_type: Option<String>,
    owner_id: Option<i64>,
    binding_role: Option<String>,
    priority: Option<i32>,
    enabled: Option<bool>,
    policy_json: Option<Value>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/backend/v3/api/prompts",
            get(list_prompts).post(create_prompt),
        )
        .route(
            "/backend/v3/api/prompts/{prompt_id}/versions",
            get(list_versions).post(create_version),
        )
        .route(
            "/backend/v3/api/prompts/versions/{version_id}/publish",
            post(publish_version),
        )
        .route(
            "/backend/v3/api/prompts/versions/{version_id}/render",
            post(render_version),
        )
        .route(
            "/backend/v3/api/prompts/{prompt_id}/bindings",
            get(list_bindings).post(create_binding),
        )
        .route(
            "/backend/v3/api/prompts/bindings/{binding_id}",
            put(update_binding),
        )
}

fn subject(ctx: &PromptsCtx) -> PromptAiSubject {
    PromptAiSubject {
        tenant_id: ctx.0.tenant_id_value(),
        organization_id: ctx.0.organization_id_value(),
        operator_id: ctx.0.user_id_value(),
    }
}

async fn list_prompts(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Query(request): Query<ListPromptsRequest>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let page_no = request.page.unwrap_or(DEFAULT_PAGE_NO).max(1);
    let page_size = request
        .page_size
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(1, MAX_PAGE_SIZE);
    let query = ListPromptsQuery {
        subject: subject(&PromptsCtx(ctx.clone())),
        keyword: request.q,
        prompt_type: request.prompt_type,
        visibility: request.visibility,
        status: request.status,
        category_id: request.category_id,
        page_no,
        page_size,
        offset: (page_no - 1) * page_size,
    };
    match state.service_host.ai_repository().list_prompts(query).await {
        Ok(items) => Ok(Json(PlusApiResult::ok(json!({ "items": items })))),
        Err(error) => Err(map_error(error)),
    }
}

async fn create_prompt(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Json(request): Json<CreatePromptRequest>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let command = CreatePromptCommand {
        subject: subject(&PromptsCtx(ctx)),
        prompt_key: request.prompt_key,
        name: request.name,
        description: request.description,
        category_id: request.category_id,
        prompt_type: request.prompt_type.unwrap_or_else(|| "general".to_string()),
        visibility: request.visibility.unwrap_or_else(|| "private".to_string()),
        tags: request.tags.unwrap_or_default(),
    };
    match state.service_host.ai_repository().create_prompt(command).await {
        Ok(item) => Ok(Json(PlusApiResult::ok(json!({ "item": item })))),
        Err(error) => Err(map_error(error)),
    }
}

async fn list_versions(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(prompt_id): Path<String>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let prompt_id = parse_id(&prompt_id)?;
    let query = ListPromptVersionsQuery {
        subject: subject(&PromptsCtx(ctx)),
        prompt_id,
    };
    match state.service_host.ai_repository().list_versions(query).await {
        Ok(items) => Ok(Json(PlusApiResult::ok(json!({ "items": items })))),
        Err(error) => Err(map_error(error)),
    }
}

async fn create_version(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(prompt_id): Path<String>,
    Json(request): Json<CreatePromptVersionRequest>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let prompt_id = parse_id(&prompt_id)?;
    let command = CreatePromptVersionCommand {
        subject: subject(&PromptsCtx(ctx)),
        prompt_id,
        version_no: request.version_no,
        title: request.title,
        content: request.content,
        variable_schema: request.variable_schema.unwrap_or_else(|| json!({})),
        output_schema: request.output_schema.unwrap_or_else(|| json!({})),
        model_constraints: request.model_constraints.unwrap_or_else(|| json!({})),
        safety_policy: request.safety_policy.unwrap_or_else(|| json!({})),
        examples_json: request.examples_json.unwrap_or_else(|| json!([])),
    };
    match state
        .service_host
        .ai_repository()
        .create_version(command)
        .await
    {
        Ok(item) => Ok(Json(PlusApiResult::ok(json!({ "item": item })))),
        Err(error) => Err(map_error(error)),
    }
}

async fn publish_version(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(version_id): Path<String>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let version_id = parse_id(&version_id)?;
    let command = PublishPromptVersionCommand {
        subject: subject(&PromptsCtx(ctx)),
        version_id,
    };
    match state
        .service_host
        .ai_repository()
        .publish_version(command)
        .await
    {
        Ok(Some(item)) => Ok(Json(PlusApiResult::ok(json!({ "item": item })))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => Err(map_error(error)),
    }
}

async fn render_version(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(version_id): Path<String>,
    Json(request): Json<RenderPromptVersionRequest>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let version_id = parse_id(&version_id)?;
    let command = RenderPromptVersionCommand {
        subject: subject(&PromptsCtx(ctx)),
        version_id,
        variables: request.variables.unwrap_or_else(|| json!({})),
    };
    match state.service_host.ai_repository().render_version(command).await {
        Ok(Some(rendered)) => Ok(Json(PlusApiResult::ok(json!({ "rendered": rendered })))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => Err(map_error(error)),
    }
}

async fn list_bindings(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(prompt_id): Path<String>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let prompt_id = parse_id(&prompt_id)?;
    let query = ListPromptBindingsQuery {
        subject: subject(&PromptsCtx(ctx)),
        prompt_id,
    };
    match state.service_host.ai_repository().list_bindings(query).await {
        Ok(items) => Ok(Json(PlusApiResult::ok(json!({ "items": items })))),
        Err(error) => Err(map_error(error)),
    }
}

async fn create_binding(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(prompt_id): Path<String>,
    Json(request): Json<CreatePromptBindingRequest>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let prompt_id = parse_id(&prompt_id)?;
    let command = CreatePromptBindingCommand {
        subject: subject(&PromptsCtx(ctx)),
        prompt_id,
        prompt_version_id: request.prompt_version_id,
        owner_type: request.owner_type,
        owner_id: request.owner_id,
        binding_role: request.binding_role,
        priority: request.priority.unwrap_or(0),
        enabled: request.enabled.unwrap_or(true),
        policy_json: request.policy_json.unwrap_or_else(|| json!({})),
    };
    match state
        .service_host
        .ai_repository()
        .create_binding(command)
        .await
    {
        Ok(item) => Ok(Json(PlusApiResult::ok(json!({ "item": item })))),
        Err(error) => Err(map_error(error)),
    }
}

async fn update_binding(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(binding_id): Path<String>,
    Json(request): Json<UpdatePromptBindingRequest>,
) -> Result<Json<PlusApiResult<Value>>, StatusCode> {
    let binding_id = parse_id(&binding_id)?;
    let prompt_version_id = request.prompt_version_id.map(|value| {
        if value.is_null() {
            None
        } else {
            value.as_i64()
        }
    });
    let command = UpdatePromptBindingCommand {
        subject: subject(&PromptsCtx(ctx)),
        binding_id,
        prompt_version_id,
        owner_type: request.owner_type,
        owner_id: request.owner_id,
        binding_role: request.binding_role,
        priority: request.priority,
        enabled: request.enabled,
        policy_json: request.policy_json,
    };
    match state
        .service_host
        .ai_repository()
        .update_binding(command)
        .await
    {
        Ok(Some(item)) => Ok(Json(PlusApiResult::ok(json!({ "item": item })))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(error) => Err(map_error(error)),
    }
}

fn parse_id(raw: &str) -> Result<i64, StatusCode> {
    raw.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)
}

fn map_error(error: PromptAiError) -> StatusCode {
    match error {
        PromptAiError::NotFound(_) => StatusCode::NOT_FOUND,
        PromptAiError::Conflict(_) => StatusCode::CONFLICT,
        PromptAiError::Validation(_) => StatusCode::BAD_REQUEST,
        PromptAiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
