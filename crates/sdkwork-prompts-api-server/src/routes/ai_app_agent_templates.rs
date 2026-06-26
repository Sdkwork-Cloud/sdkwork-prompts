use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sdkwork_intelligence_prompts_ai_contract::{
    domain::AgentPromptTemplateRecord, ports::AgentPromptTemplateListQuery, PromptAiError,
    PromptAiRepository,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::context::PromptsCtx;
use crate::AppState;

const DEFAULT_LIMIT: u32 = 50;
const MAX_LIMIT: u32 = 200;

#[derive(Debug, Default, Deserialize)]
struct AgentTemplateListQuery {
    limit: Option<u32>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/app/v3/api/prompts/agent_templates",
            get(list_agent_templates),
        )
        .route(
            "/app/v3/api/prompts/agent_templates/{templateId}",
            get(get_agent_template),
        )
}

async fn list_agent_templates(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<AgentTemplateListQuery>,
) -> Result<Json<Value>, StatusCode> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let list_query = AgentPromptTemplateListQuery {
        tenant_id: ctx.tenant_id_value(),
        organization_id: ctx.organization_id_value(),
        limit,
    };
    match state
        .service_host
        .ai_repository()
        .list_agent_prompt_templates(list_query)
        .await
    {
        Ok(items) => Ok(Json(json!({
            "items": items.iter().map(agent_template_json).collect::<Vec<_>>(),
        }))),
        Err(error) => Err(map_error(error)),
    }
}

async fn get_agent_template(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(template_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let id = parse_id(&template_id)?;
    match state
        .service_host
        .ai_repository()
        .get_agent_prompt_template(ctx.tenant_id_value(), id)
        .await
    {
        Ok(record) => Ok(Json(agent_template_json(&record))),
        Err(error) => Err(map_error(error)),
    }
}

fn agent_template_json(record: &AgentPromptTemplateRecord) -> Value {
    json!({
        "id": record.id.to_string(),
        "uuid": record.uuid,
        "promptId": record.prompt_id,
        "code": record.code,
        "displayName": record.display_name,
        "description": record.description,
        "promptKind": record.prompt_kind,
        "templateFormat": record.template_format,
        "templateBody": record.template_body,
        "safetyProfileId": record.safety_profile_id,
        "status": record.status,
        "visibility": record.visibility,
    })
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
