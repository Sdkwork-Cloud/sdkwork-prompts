use axum::{
    extract::{Path, Query, State},
    routing::{get, patch, post, put},
    Json, Router,
};
use sdkwork_intelligence_prm_service::domain::commands::*;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::context::{page_json, PromptsCtx};
use crate::dto::{ApiResponse, CreateReplyRequest, CreateTopicRequest};
use crate::AppState;

#[derive(Debug, Deserialize)]
struct CursorQuery {
    cursor: Option<String>,
    limit: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct BoardTopicsQuery {
    board_id: Option<i64>,
    cursor: Option<String>,
    limit: Option<u16>,
}

#[derive(Debug, Deserialize)]
struct FeedQuery {
    feed_type: Option<String>,
    feed_owner_id: Option<String>,
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
        .route("/app/v3/api/prompts/nodes/tree", get(list_node_tree))
        .route("/app/v3/api/prompts/boards/{board_id}/topics", get(list_board_topics))
        .route("/app/v3/api/prompts/topics", get(list_topics).post(create_topic))
        .route(
            "/app/v3/api/prompts/topics/{topic_id}",
            get(retrieve_topic).patch(update_topic).delete(delete_topic),
        )
        .route(
            "/app/v3/api/prompts/topics/{topic_id}/replies",
            get(list_replies).post(create_reply),
        )
        .route(
            "/app/v3/api/prompts/replies/{reply_id}",
            patch(update_reply).delete(delete_reply),
        )
        .route("/app/v3/api/prompts/topics/{topic_id}/revisions", get(list_topic_revisions))
        .route("/app/v3/api/prompts/replies/{reply_id}/revisions", get(list_reply_revisions))
        .route(
            "/app/v3/api/prompts/questions/{topic_id}/accepted_reply",
            put(accept_reply).delete(clear_accepted_reply),
        )
        .route("/app/v3/api/prompts/polls/{poll_id}/votes", post(create_poll_vote))
        .route("/app/v3/api/prompts/reactions", post(create_reaction))
        .route("/app/v3/api/prompts/votes", post(create_vote))
        .route("/app/v3/api/prompts/bookmarks", post(create_bookmark))
        .route("/app/v3/api/prompts/read_state/topics/{topic_id}", patch(update_read_state))
        .route("/app/v3/api/prompts/reports", post(create_report))
        .route("/app/v3/api/prompts/feed", get(list_feed))
        .route("/app/v3/api/prompts/search", get(search))
}

async fn list_node_tree(State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_node_tree(
        &ctx,
        ListNodeTreeCommand {
            space_id: None,
            parent_id: None,
        },
    ) {
        Ok(nodes) => Json(ApiResponse::ok(json!({ "items": nodes }))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_board_topics(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(board_id): Path<i64>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topics(
        &ctx,
        ListTopicsCommand {
            board_id: Some(board_id),
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
            sort: Some("latest".to_string()),
            status_filter: None,
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_topics(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<BoardTopicsQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topics(
        &ctx,
        ListTopicsCommand {
            board_id: query.board_id,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
            sort: Some("latest".to_string()),
            status_filter: None,
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(req): Json<CreateTopicRequest>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_topic(
        &ctx,
        CreateTopicCommand {
            board_id: req.board_id,
            title: req.title,
            body_format: req.body_format.unwrap_or_else(|| "markdown".to_string()),
            body: req.body,
            tag_ids: vec![],
            prefix_id: None,
            topic_type: req.topic_type,
            visibility: req.visibility,
        },
    ) {
        Ok(topic) => Json(ApiResponse::ok(json!(topic))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn retrieve_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().retrieve_topic(&ctx, topic_id) {
        Ok(topic) => Json(ApiResponse::ok(json!(topic))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn update_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().update_topic(
        &ctx,
        UpdateTopicCommand {
            topic_id,
            title: body.get("title").and_then(Value::as_str).map(str::to_string),
            body: body.get("body").and_then(Value::as_str).map(str::to_string),
            body_format: body
                .get("bodyFormat")
                .or_else(|| body.get("body_format"))
                .and_then(Value::as_str)
                .map(str::to_string),
            edit_reason: body
                .get("editReason")
                .or_else(|| body.get("edit_reason"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
    ) {
        Ok(topic) => Json(ApiResponse::ok(json!(topic))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn delete_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().delete_topic(
        &ctx,
        DeleteTopicCommand {
            topic_id,
            reason: None,
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_replies(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
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

async fn create_reply(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
    Json(req): Json<CreateReplyRequest>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_reply(
        &ctx,
        CreateReplyCommand {
            topic_id,
            parent_reply_id: req.parent_reply_id,
            body_format: req.body_format.unwrap_or_else(|| "markdown".to_string()),
            body: req.body,
        },
    ) {
        Ok(reply) => Json(ApiResponse::ok(json!(reply))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn update_reply(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(reply_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().update_reply(
        &ctx,
        UpdateReplyCommand {
            reply_id,
            body: body.get("body").and_then(Value::as_str).map(str::to_string),
            body_format: body
                .get("bodyFormat")
                .or_else(|| body.get("body_format"))
                .and_then(Value::as_str)
                .map(str::to_string),
            edit_reason: body
                .get("editReason")
                .or_else(|| body.get("edit_reason"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
    ) {
        Ok(reply) => Json(ApiResponse::ok(json!(reply))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn delete_reply(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(reply_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().delete_reply(
        &ctx,
        DeleteReplyCommand {
            reply_id,
            reason: None,
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_topic_revisions(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topic_revisions(
        &ctx,
        ListTopicRevisionsCommand {
            topic_id,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_reply_revisions(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(reply_id): Path<i64>,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_reply_revisions(
        &ctx,
        ListReplyRevisionsCommand {
            reply_id,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn accept_reply(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    let reply_id = body
        .get("replyId")
        .or_else(|| body.get("reply_id"))
        .and_then(Value::as_i64)
        .unwrap_or_default();
    match state.service_host.service().accept_reply(
        &ctx,
        AcceptReplyCommand { topic_id, reply_id },
    ) {
        Ok(topic) => Json(ApiResponse::ok(json!(topic))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn clear_accepted_reply(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().clear_accepted_reply(
        &ctx,
        ClearAcceptedReplyCommand { topic_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_poll_vote(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(poll_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    let option_ids = body
        .get("optionIds")
        .or_else(|| body.get("option_ids"))
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_i64).collect())
        .unwrap_or_default();
    match state.service_host.service().create_poll_vote(
        &ctx,
        CreatePollVoteCommand { poll_id, option_ids },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_reaction(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_reaction(
        &ctx,
        CreateReactionCommand {
            target_type: body
                .get("targetType")
                .or_else(|| body.get("target_type"))
                .and_then(Value::as_str)
                .unwrap_or("topic")
                .to_string(),
            target_id: body
                .get("targetId")
                .or_else(|| body.get("target_id"))
                .and_then(Value::as_i64)
                .unwrap_or_default(),
            reaction_type: body
                .get("reactionType")
                .or_else(|| body.get("reaction_type"))
                .and_then(Value::as_str)
                .unwrap_or("like")
                .to_string(),
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_vote(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_vote(
        &ctx,
        CreateVoteCommand {
            target_type: body
                .get("targetType")
                .or_else(|| body.get("target_type"))
                .and_then(Value::as_str)
                .unwrap_or("topic")
                .to_string(),
            target_id: body
                .get("targetId")
                .or_else(|| body.get("target_id"))
                .and_then(Value::as_i64)
                .unwrap_or_default(),
            vote_value: body
                .get("voteValue")
                .or_else(|| body.get("vote_value"))
                .and_then(Value::as_i64)
                .unwrap_or(1) as i32,
            reason_code: body
                .get("reasonCode")
                .or_else(|| body.get("reason_code"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_bookmark(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().update_bookmark(
        &ctx,
        UpdateBookmarkCommand {
            target_type: body
                .get("targetType")
                .or_else(|| body.get("target_type"))
                .and_then(Value::as_str)
                .unwrap_or("topic")
                .to_string(),
            target_id: body
                .get("targetId")
                .or_else(|| body.get("target_id"))
                .and_then(Value::as_i64)
                .unwrap_or_default(),
            note: body.get("note").and_then(Value::as_str).map(str::to_string),
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn update_read_state(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().update_read_state(
        &ctx,
        UpdateReadStateCommand {
            topic_id,
            last_read_reply_id: body
                .get("lastReadReplyId")
                .or_else(|| body.get("last_read_reply_id"))
                .and_then(Value::as_i64),
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_report(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_report(
        &ctx,
        CreateReportCommand {
            target_type: body
                .get("targetType")
                .or_else(|| body.get("target_type"))
                .and_then(Value::as_str)
                .unwrap_or("topic")
                .to_string(),
            target_id: body
                .get("targetId")
                .or_else(|| body.get("target_id"))
                .and_then(Value::as_i64)
                .unwrap_or_default(),
            reason_code: body
                .get("reasonCode")
                .or_else(|| body.get("reason_code"))
                .and_then(Value::as_str)
                .unwrap_or("other")
                .to_string(),
            description: body.get("description").and_then(Value::as_str).map(str::to_string),
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_feed(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<FeedQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_feed(
        &ctx,
        ListFeedCommand {
            feed_type: query.feed_type,
            feed_owner_id: query.feed_owner_id,
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
