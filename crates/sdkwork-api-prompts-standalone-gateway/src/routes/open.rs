use axum::{extract::State, response::Response, routing::get, Router};
use sdkwork_intelligence_prompts_ai_contract::{
    commands::{ListPromptsQuery, PromptAiSubject},
    PromptAiRepository,
};
use serde_json::{json, Value};

use crate::response::{anonymous_ok_json, anonymous_prompt_error, cursor_page_info, page_data};
use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/prompts/v3/api/prompts/catalog", get(list_prompt_catalog))
}

async fn list_prompt_catalog(State(state): State<AppState>) -> Response {
    let tenant_id = std::env::var("SDKWORK_PROMPTS_DEFAULT_TENANT_ID")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(100_001);
    let subject = PromptAiSubject {
        tenant_id,
        organization_id: 0,
        operator_id: 0,
    };
    let query = ListPromptsQuery {
        subject,
        keyword: None,
        prompt_type: None,
        visibility: Some("public".to_string()),
        status: Some("active".to_string()),
        category_id: None,
        page_no: 1,
        page_size: 200,
        offset: 0,
    };
    match state.service_host.ai_repository().list_prompts(query).await {
        Ok(items) => {
            let mapped: Vec<Value> = items
                .iter()
                .map(|item| {
                    json!({
                        "key": item.prompt_key,
                        "name": item.name,
                        "description": item.description,
                    })
                })
                .collect();
            anonymous_ok_json(page_data(mapped, cursor_page_info(None, false)))
        }
        Err(error) => anonymous_prompt_error(error),
    }
}
