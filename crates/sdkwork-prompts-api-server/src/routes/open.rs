use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use sdkwork_intelligence_prm_service::domain::commands::*;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::context::{page_json, PromptsCtx};
use crate::dto::ApiResponse;
use crate::AppState;

#[derive(Debug, Deserialize)]
struct CursorQuery {
    cursor: Option<String>,
    limit: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
    board_id: Option<i64>,
    cursor: Option<String>,
    limit: Option<u16>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/prompts/v3/api/sites/{site_slug}/boards", get(list_boards))
        .route(
            "/prompts/v3/api/sites/{site_slug}/boards/{board_id}/topics",
            get(list_board_topics),
        )
        .route("/prompts/v3/api/sites/{site_slug}/topics", get(list_topics))
        .route(
            "/prompts/v3/api/sites/{site_slug}/topics/{topic_id}",
            get(retrieve_topic),
        )
        .route(
            "/prompts/v3/api/sites/{site_slug}/topics/by_slug/{topic_slug}",
            get(retrieve_topic_by_slug),
        )
        .route(
            "/prompts/v3/api/sites/{site_slug}/topics/{topic_id}/replies",
            get(list_replies),
        )
        .route("/prompts/v3/api/sites/{site_slug}/tags", get(list_tags))
        .route("/prompts/v3/api/sites/{site_slug}/search", get(search))
}

async fn list_boards(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(_site_slug): Path<String>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_nodes(
        &ctx,
        ListNodesCommand {
            space_id: None,
            node_type: Some("board".to_string()),
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_board_topics(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path((_site_slug, board_id)): Path<(String, i64)>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topics(
        &ctx,
        ListTopicsCommand {
            board_id: Some(board_id),
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
            sort: Some("latest".to_string()),
            status_filter: Some("visible".to_string()),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_topics(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(_site_slug): Path<String>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topics(
        &ctx,
        ListTopicsCommand {
            board_id: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
            sort: Some("latest".to_string()),
            status_filter: Some("visible".to_string()),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn retrieve_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path((_site_slug, topic_id)): Path<(String, i64)>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().retrieve_topic(&ctx, topic_id) {
        Ok(topic) => Json(ApiResponse::ok(json!(topic))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn retrieve_topic_by_slug(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path((_site_slug, topic_slug)): Path<(String, String)>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().retrieve_topic_by_slug(
        &ctx,
        RetrieveTopicBySlugCommand {
            slug: topic_slug,
            board_id: None,
        },
    ) {
        Ok(topic) => Json(ApiResponse::ok(json!(topic))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_replies(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path((_site_slug, topic_id)): Path<(String, i64)>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_replies(
        &ctx,
        ListRepliesCommand {
            topic_id,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_tags(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(_site_slug): Path<String>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_tags(
        &ctx,
        ListTagsCommand {
            space_id: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn search(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(_site_slug): Path<String>,
    Query(query): Query<SearchQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().query_search(
        &ctx,
        QuerySearchCommand {
            query: query.q.unwrap_or_default(),
            board_id: query.board_id,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}
