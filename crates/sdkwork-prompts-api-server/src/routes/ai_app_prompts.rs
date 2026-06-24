use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sdkwork_intelligence_prompts_ai_contract::{
    commands::{
        CreatePromptCommand, CreatePromptVersionCommand, ListPromptsQuery, ListPromptVersionsQuery,
        PromptAiItem, PromptAiSubject, PromptAiVersionItem, UpdatePromptCommand,
    },
    PromptAiError, PromptAiRepository,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::context::PromptsCtx;
use crate::AppState;

const DEFAULT_LIMIT: i64 = 20;
const MAX_LIMIT: i64 = 100;

#[derive(Debug, Default, Deserialize)]
struct TemplateListQuery {
    cursor: Option<String>,
    limit: Option<i64>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TemplateCreateRequest {
    key: String,
    name: String,
    description: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TemplateUpdateRequest {
    name: Option<String>,
    description: Option<String>,
    status: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TemplateVersionCreateRequest {
    version_label: String,
    content: String,
    model_hint: Option<String>,
    variables: Option<Vec<Value>>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/app/v3/api/prompts/templates",
            get(list_templates).post(create_template),
        )
        .route(
            "/app/v3/api/prompts/templates/{template_id}",
            get(get_template).patch(update_template),
        )
        .route(
            "/app/v3/api/prompts/templates/{template_id}/versions",
            get(list_template_versions).post(create_template_version),
        )
}

fn subject(ctx: &PromptsCtx) -> PromptAiSubject {
    PromptAiSubject {
        tenant_id: ctx.0.tenant_id_value(),
        organization_id: ctx.0.organization_id_value(),
        operator_id: ctx.0.user_id_value(),
    }
}

async fn list_templates(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<TemplateListQuery>,
) -> Result<Json<Value>, StatusCode> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = query
        .cursor
        .as_deref()
        .and_then(|value| value.parse::<i64>().ok())
        .unwrap_or(0)
        .max(0);
    let page_no = offset / limit + 1;
    let list_query = ListPromptsQuery {
        subject: subject(&PromptsCtx(ctx)),
        keyword: None,
        prompt_type: None,
        visibility: None,
        status: query.status,
        category_id: None,
        page_no,
        page_size: limit,
        offset,
    };
    match state.service_host.ai_repository().list_prompts(list_query).await {
        Ok(items) => {
            let mapped: Vec<Value> = items.iter().map(template_json).collect();
            let next_cursor = if mapped.len() as i64 == limit {
                Some((offset + limit).to_string())
            } else {
                None
            };
            Ok(Json(json!({
                "items": mapped,
                "next_cursor": next_cursor,
            })))
        }
        Err(error) => Err(map_error(error)),
    }
}

async fn create_template(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Json(request): Json<TemplateCreateRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let command = CreatePromptCommand {
        subject: subject(&PromptsCtx(ctx)),
        prompt_key: request.key,
        name: request.name,
        description: request.description,
        category_id: None,
        prompt_type: "template".to_string(),
        visibility: "tenant".to_string(),
        tags: request.tags.unwrap_or_default(),
    };
    match state.service_host.ai_repository().create_prompt(command).await {
        Ok(item) => Ok((StatusCode::CREATED, Json(template_json(&item)))),
        Err(error) => Err(map_error(error)),
    }
}

async fn get_template(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(template_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let prompt_id = parse_id(&template_id)?;
    match state
        .service_host
        .ai_repository()
        .get_prompt(subject(&PromptsCtx(ctx)).tenant_id, prompt_id)
        .await
    {
        Ok(record) => Ok(Json(template_json_from_record(&record))),
        Err(error) => Err(map_error(error)),
    }
}

async fn update_template(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(template_id): Path<String>,
    Json(request): Json<TemplateUpdateRequest>,
) -> Result<Json<Value>, StatusCode> {
    let command = UpdatePromptCommand {
        subject: subject(&PromptsCtx(ctx)),
        prompt_id: parse_id(&template_id)?,
        name: request.name,
        description: request.description,
        tags: request.tags,
        status: request.status,
    };
    match state.service_host.ai_repository().update_prompt(command).await {
        Ok(item) => Ok(Json(template_json(&item))),
        Err(error) => Err(map_error(error)),
    }
}

async fn list_template_versions(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(template_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let query = ListPromptVersionsQuery {
        subject: subject(&PromptsCtx(ctx)),
        prompt_id: parse_id(&template_id)?,
    };
    match state.service_host.ai_repository().list_versions(query).await {
        Ok(items) => Ok(Json(json!({
            "items": items.iter().map(version_json).collect::<Vec<_>>(),
        }))),
        Err(error) => Err(map_error(error)),
    }
}

async fn create_template_version(
    State(state): State<AppState>,
    PromptsCtx(ctx): PromptsCtx,
    Path(template_id): Path<String>,
    Json(request): Json<TemplateVersionCreateRequest>,
) -> Result<(StatusCode, Json<Value>), StatusCode> {
    let variable_schema = variables_to_schema(request.variables.as_deref());
    let version_label = request.version_label;
    let command = CreatePromptVersionCommand {
        subject: subject(&PromptsCtx(ctx)),
        prompt_id: parse_id(&template_id)?,
        version_no: version_label.clone(),
        title: version_label,
        content: request.content,
        variable_schema,
        output_schema: json!({}),
        model_constraints: request
            .model_hint
            .map(|hint| json!({ "modelHint": hint }))
            .unwrap_or_else(|| json!({})),
        safety_policy: json!({}),
        examples_json: json!([]),
    };
    match state.service_host.ai_repository().create_version(command).await {
        Ok(item) => Ok((StatusCode::CREATED, Json(version_json(&item)))),
        Err(error) => Err(map_error(error)),
    }
}

fn template_json(item: &PromptAiItem) -> Value {
    json!({
        "id": item.id.to_string(),
        "key": item.prompt_key,
        "name": item.name,
        "description": item.description,
        "status": template_status_label(&item.status),
        "tags": item.tags,
        "latest_version_id": item.latest_version_id.map(|id| id.to_string()),
        "updated_at": item.updated_at,
    })
}

fn template_json_from_record(record: &sdkwork_intelligence_prompts_ai_contract::PromptRecord) -> Value {
    let status = match record.status {
        1 => "active",
        0 => "archived",
        _ => "draft",
    };
    json!({
        "id": record.id.to_string(),
        "key": record.prompt_key,
        "name": record.name,
        "description": record.description,
        "status": status,
        "tags": [],
        "latest_version_id": record.latest_version_id.map(|id| id.to_string()),
        "updated_at": "",
    })
}

fn version_json(item: &PromptAiVersionItem) -> Value {
    json!({
        "id": item.id.to_string(),
        "template_id": item.prompt_id.to_string(),
        "version_label": item.version_no,
        "content": item.content,
        "model_hint": item.model_constraints.get("modelHint").and_then(Value::as_str),
        "status": version_status_label(&item.lifecycle_status),
        "variables": schema_to_variables(&item.variable_schema),
    })
}

fn template_status_label(status: &str) -> &str {
    match status {
        "enabled" | "active" => "active",
        "disabled" | "archived" => "archived",
        _ => "draft",
    }
}

fn version_status_label(status: &str) -> &str {
    match status {
        "published" | "active" => "active",
        "archived" | "deprecated" => "archived",
        _ => "draft",
    }
}

fn variables_to_schema(variables: Option<&[Value]>) -> Value {
    let Some(items) = variables else {
        return json!({});
    };
    let mut properties = serde_json::Map::new();
    for item in items {
        let Some(name) = item.get("name").and_then(Value::as_str) else {
            continue;
        };
        properties.insert(
            name.to_string(),
            json!({
                "type": item.get("var_type").and_then(Value::as_str).unwrap_or("string"),
                "required": item.get("required").and_then(Value::as_bool).unwrap_or(false),
                "default": item.get("default_value"),
                "description": item.get("description"),
            }),
        );
    }
    json!({ "properties": properties })
}

fn schema_to_variables(schema: &Value) -> Vec<Value> {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return vec![];
    };
    properties
        .iter()
        .map(|(name, definition)| {
            json!({
                "name": name,
                "var_type": definition.get("type").and_then(Value::as_str).unwrap_or("string"),
                "required": definition.get("required").and_then(Value::as_bool).unwrap_or(false),
                "default_value": definition.get("default"),
                "description": definition.get("description"),
            })
        })
        .collect()
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
