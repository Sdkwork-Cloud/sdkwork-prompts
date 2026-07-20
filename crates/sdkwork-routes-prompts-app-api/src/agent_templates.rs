use axum::{
    extract::{Path, Query, State},
    response::Response,
    routing::get,
    Router,
};
use sdkwork_intelligence_prompts_ai_contract::{
    domain::AgentPromptTemplateRecord, ports::AgentPromptTemplateListQuery,
};
use sdkwork_web_core::WebFrameworkErrorKind;
use serde::Deserialize;
use serde_json::{json, Value};

use sdkwork_prompts_web_context::{
    cursor_page_info, map_prompt_error, ok_json, page_data, resource_data, status_problem,
    AppState, PromptsCtx, PromptsRequestContext,
};

const DEFAULT_LIMIT: u32 = 50;
const MAX_LIMIT: u32 = 200;

#[derive(Debug, Default, Deserialize)]
struct AgentTemplateListQuery {
    page_size: Option<u32>,
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
) -> Response {
    let limit = query.page_size.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let list_query = AgentPromptTemplateListQuery {
        tenant_id: ctx.tenant_id_value(),
        organization_id: ctx.organization_id_value(),
        limit,
    };
    match state
        .ai_repository()
        .list_agent_prompt_templates(list_query)
        .await
    {
        Ok(items) => {
            let mapped: Vec<Value> = items.iter().map(agent_template_json).collect();
            ok_json(&ctx, page_data(mapped, cursor_page_info(None, false)))
        }
        Err(error) => map_prompt_error(&ctx, error),
    }
}

async fn get_agent_template(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(template_id): Path<String>,
) -> Response {
    let id = match parse_id(&template_id, &ctx) {
        Ok(id) => id,
        Err(response) => return response,
    };
    match state
        .ai_repository()
        .get_agent_prompt_template(ctx.tenant_id_value(), id)
        .await
    {
        Ok(record) => ok_json(&ctx, resource_data(agent_template_json(&record))),
        Err(error) => map_prompt_error(&ctx, error),
    }
}

fn agent_template_json(record: &AgentPromptTemplateRecord) -> Value {
    json!({
        "id": record.id.to_string(),
        "uuid": record.uuid,
        "promptId": record.prompt_id.to_string(),
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

fn parse_id(raw: &str, ctx: &PromptsRequestContext) -> Result<i64, Response> {
    raw.parse::<i64>().map_err(|_| {
        status_problem(
            ctx,
            WebFrameworkErrorKind::BadRequest,
            "invalid resource id",
        )
    })
}
