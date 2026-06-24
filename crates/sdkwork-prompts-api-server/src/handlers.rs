use axum::Json;
use serde_json::{json, Value};

pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "sdkwork-prompts",
        "version": "0.1.0"
    }))
}
