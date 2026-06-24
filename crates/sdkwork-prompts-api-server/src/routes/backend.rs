use axum::{
    extract::{Path, Query, State},
    routing::{get, patch, post},
    Json, Router,
};
use sdkwork_intelligence_prompts_service::domain::commands::*;
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
struct TopicsQuery {
    board_id: Option<i64>,
    cursor: Option<String>,
    limit: Option<u16>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/backend/v3/api/prompts/nodes", get(list_nodes).post(create_node))
        .route(
            "/backend/v3/api/prompts/nodes/{node_id}",
            patch(update_node).delete(delete_node),
        )
        .route(
            "/backend/v3/api/prompts/topic_prefixes",
            get(list_topic_prefixes).post(create_topic_prefix),
        )
        .route("/backend/v3/api/prompts/topics", get(list_topics))
        .route(
            "/backend/v3/api/prompts/topics/{topic_id}",
            get(retrieve_topic).patch(update_topic).delete(delete_topic),
        )
        .route("/backend/v3/api/prompts/topics/{topic_id}/pin", post(pin_topic).delete(unpin_topic))
        .route(
            "/backend/v3/api/prompts/topics/{topic_id}/feature",
            post(feature_topic).delete(unfeature_topic),
        )
        .route(
            "/backend/v3/api/prompts/topics/{topic_id}/lock",
            post(lock_topic).delete(unlock_topic),
        )
        .route("/backend/v3/api/prompts/topics/{topic_id}/move", post(move_topic))
        .route("/backend/v3/api/prompts/moderation/queue", get(list_moderation_queue))
        .route(
            "/backend/v3/api/prompts/moderation/cases",
            get(list_moderation_cases).post(create_moderation_case),
        )
        .route(
            "/backend/v3/api/prompts/moderation/cases/{case_id}",
            get(retrieve_moderation_case),
        )
        .route(
            "/backend/v3/api/prompts/moderation/cases/{case_id}/decisions",
            post(create_moderation_decision),
        )
        .route(
            "/backend/v3/api/prompts/sanctions",
            get(list_sanctions).post(create_sanction),
        )
        .route("/backend/v3/api/prompts/sanctions/{sanction_id}", patch(update_sanction))
        .route(
            "/backend/v3/api/prompts/reputation/rules",
            get(list_reputation_rules).post(create_reputation_rule),
        )
        .route("/backend/v3/api/prompts/reputation/ledger", get(list_reputation_ledger))
        .route(
            "/backend/v3/api/prompts/trust_levels",
            get(list_trust_levels).post(create_trust_level),
        )
        .route("/backend/v3/api/prompts/badges", get(list_badges).post(create_badge))
        .route("/backend/v3/api/prompts/stats/boards", get(list_board_stats))
        .route("/backend/v3/api/prompts/stats/topics", get(list_topic_stats))
        .route("/backend/v3/api/prompts/search/reindex", post(reindex_search))
        .route("/backend/v3/api/prompts/audit/actions", get(list_audit_actions))
}

async fn list_nodes(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_nodes(
        &ctx,
        ListNodesCommand {
            space_id: None,
            node_type: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_node(State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx, Json(body): Json<Value>) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_node(
        &ctx,
        CreateNodeCommand {
            space_id: body
                .get("spaceId")
                .or_else(|| body.get("space_id"))
                .and_then(Value::as_i64)
                .unwrap_or(1),
            parent_id: body.get("parentId").or_else(|| body.get("parent_id")).and_then(Value::as_i64),
            node_type: body
                .get("nodeType")
                .or_else(|| body.get("node_type"))
                .and_then(Value::as_str)
                .unwrap_or("board")
                .to_string(),
            slug: body.get("slug").and_then(Value::as_str).unwrap_or("node").to_string(),
            name: body
                .get("name")
                .or_else(|| body.get("title"))
                .and_then(Value::as_str)
                .unwrap_or("Node")
                .to_string(),
            description: body.get("description").and_then(Value::as_str).map(str::to_string),
            sort_order: body
                .get("sortOrder")
                .or_else(|| body.get("sort_order"))
                .and_then(Value::as_i64)
                .unwrap_or(0) as i32,
        },
    ) {
        Ok(node) => Json(ApiResponse::ok(json!(node))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn update_node(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(node_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().update_node(
        &ctx,
        UpdateNodeCommand {
            node_id,
            name: body
                .get("name")
                .or_else(|| body.get("title"))
                .and_then(Value::as_str)
                .map(str::to_string),
            description: body.get("description").and_then(Value::as_str).map(str::to_string),
            sort_order: body
                .get("sortOrder")
                .or_else(|| body.get("sort_order"))
                .and_then(Value::as_i64)
                .map(|v| v as i32),
            parent_id: body.get("parentId").or_else(|| body.get("parent_id")).and_then(Value::as_i64),
        },
    ) {
        Ok(node) => Json(ApiResponse::ok(json!(node))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn delete_node(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(node_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().delete_node(
        &ctx,
        DeleteNodeCommand { node_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_topic_prefixes(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topic_prefixes(
        &ctx,
        ListTopicPrefixesCommand {
            board_id: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_topic_prefix(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_topic_prefix(
        &ctx,
        CreateTopicPrefixCommand {
            board_id: body
                .get("boardId")
                .or_else(|| body.get("board_id"))
                .and_then(Value::as_i64)
                .unwrap_or_default(),
            code: body
                .get("code")
                .or_else(|| body.get("label"))
                .and_then(Value::as_str)
                .unwrap_or("prefix")
                .to_string(),
            label: body.get("label").and_then(Value::as_str).unwrap_or("Prefix").to_string(),
            color: body.get("color").and_then(Value::as_str).map(str::to_string),
            sort_order: body
                .get("sortOrder")
                .or_else(|| body.get("sort_order"))
                .and_then(Value::as_i64)
                .unwrap_or(0) as i32,
            required_trust_level: body
                .get("requiredTrustLevel")
                .or_else(|| body.get("required_trust_level"))
                .and_then(Value::as_i64)
                .map(|v| v as i32),
        },
    ) {
        Ok(prefix) => Json(ApiResponse::ok(json!(prefix))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_topics(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<TopicsQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topics(
        &ctx,
        ListTopicsCommand {
            board_id: query.board_id,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
            sort: None,
            status_filter: None,
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
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

async fn pin_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().pin_topic(
        &ctx,
        PinTopicCommand { topic_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn unpin_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().unpin_topic(
        &ctx,
        PinTopicCommand { topic_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn feature_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().feature_topic(
        &ctx,
        FeatureTopicCommand { topic_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn unfeature_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().unfeature_topic(
        &ctx,
        FeatureTopicCommand { topic_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn lock_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().lock_topic(
        &ctx,
        LockTopicCommand { topic_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn unlock_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().unlock_topic(
        &ctx,
        LockTopicCommand { topic_id },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn move_topic(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(topic_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().move_topic(
        &ctx,
        MoveTopicCommand {
            topic_id,
            target_board_id: body
                .get("targetBoardId")
                .or_else(|| body.get("target_board_id"))
                .and_then(Value::as_i64)
                .unwrap_or_default(),
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_moderation_queue(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_moderation_queue(
        &ctx,
        ListModerationQueueCommand {
            status_filter: None,
            severity_filter: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_moderation_cases(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_moderation_cases(
        &ctx,
        ListModerationCasesCommand {
            status_filter: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_moderation_case(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_moderation_case(
        &ctx,
        CreateModerationCaseCommand {
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
            severity: body
                .get("severity")
                .or_else(|| body.get("reasonCode"))
                .or_else(|| body.get("reason_code"))
                .and_then(Value::as_str)
                .unwrap_or("medium")
                .to_string(),
            summary: body
                .get("summary")
                .or_else(|| body.get("description"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
    ) {
        Ok(case) => Json(ApiResponse::ok(json!(case))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn retrieve_moderation_case(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(case_id): Path<i64>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().retrieve_moderation_case(
        &ctx,
        RetrieveModerationCaseCommand { case_id },
    ) {
        Ok(case) => Json(ApiResponse::ok(json!(case))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_moderation_decision(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(case_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_moderation_decision(
        &ctx,
        CreateModerationDecisionCommand {
            case_id,
            decision_action: body
                .get("decisionAction")
                .or_else(|| body.get("decision_action"))
                .or_else(|| body.get("decision"))
                .and_then(Value::as_str)
                .unwrap_or("approve")
                .to_string(),
            reason_code: body
                .get("reasonCode")
                .or_else(|| body.get("reason_code"))
                .and_then(Value::as_str)
                .unwrap_or("manual_review")
                .to_string(),
            note: body
                .get("note")
                .or_else(|| body.get("notes"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
    ) {
        Ok(decision) => Json(ApiResponse::ok(json!(decision))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_sanctions(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_sanctions(
        &ctx,
        ListSanctionsCommand {
            user_id: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_sanction(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_sanction(
        &ctx,
        CreateSanctionCommand {
            user_id: body
                .get("userId")
                .or_else(|| body.get("user_id"))
                .and_then(Value::as_i64)
                .unwrap_or_default(),
            case_id: body.get("caseId").or_else(|| body.get("case_id")).and_then(Value::as_i64),
            decision_id: body
                .get("decisionId")
                .or_else(|| body.get("decision_id"))
                .and_then(Value::as_i64),
            sanction_type: body
                .get("sanctionType")
                .or_else(|| body.get("sanction_type"))
                .and_then(Value::as_str)
                .unwrap_or("mute")
                .to_string(),
            reason_code: body
                .get("reasonCode")
                .or_else(|| body.get("reason_code"))
                .or_else(|| body.get("reason"))
                .and_then(Value::as_str)
                .unwrap_or("policy")
                .to_string(),
            starts_at: body
                .get("startsAt")
                .or_else(|| body.get("starts_at"))
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string()),
            expires_at: body
                .get("expiresAt")
                .or_else(|| body.get("expires_at"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
    ) {
        Ok(sanction) => Json(ApiResponse::ok(json!(sanction))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn update_sanction(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Path(sanction_id): Path<i64>,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().update_sanction(
        &ctx,
        UpdateSanctionCommand {
            sanction_id,
            expires_at: body
                .get("expiresAt")
                .or_else(|| body.get("expires_at"))
                .and_then(Value::as_str)
                .map(str::to_string),
        },
    ) {
        Ok(sanction) => Json(ApiResponse::ok(json!(sanction))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_reputation_rules(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_reputation_rules(
        &ctx,
        ListReputationRulesCommand {
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_reputation_rule(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_reputation_rule(
        &ctx,
        CreateReputationRuleCommand {
            code: body
                .get("code")
                .or_else(|| body.get("eventType"))
                .or_else(|| body.get("event_type"))
                .and_then(Value::as_str)
                .unwrap_or("post_created")
                .to_string(),
            event_type: body
                .get("eventType")
                .or_else(|| body.get("event_type"))
                .and_then(Value::as_str)
                .unwrap_or("post")
                .to_string(),
            points: body.get("points").and_then(Value::as_i64).unwrap_or(1),
            daily_limit: body
                .get("dailyLimit")
                .or_else(|| body.get("daily_limit"))
                .or_else(|| body.get("dailyCap"))
                .or_else(|| body.get("daily_cap"))
                .and_then(Value::as_i64),
            rule_json: body
                .get("ruleJson")
                .or_else(|| body.get("rule_json"))
                .cloned()
                .unwrap_or_else(|| json!({})),
        },
    ) {
        Ok(rule) => Json(ApiResponse::ok(json!(rule))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_reputation_ledger(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_reputation_ledger(
        &ctx,
        ListReputationLedgerCommand {
            user_id: None,
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_trust_levels(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_trust_levels(
        &ctx,
        ListTrustLevelsCommand {
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_trust_level(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_trust_level(
        &ctx,
        CreateTrustLevelCommand {
            level_no: body
                .get("levelNo")
                .or_else(|| body.get("level_no"))
                .or_else(|| body.get("level"))
                .and_then(Value::as_i64)
                .unwrap_or(1) as i32,
            code: body
                .get("code")
                .or_else(|| body.get("name"))
                .and_then(Value::as_str)
                .unwrap_or("member")
                .to_string(),
            name: body.get("name").and_then(Value::as_str).unwrap_or("Member").to_string(),
            threshold_rules: body
                .get("thresholdRules")
                .or_else(|| body.get("threshold_rules"))
                .cloned()
                .unwrap_or_else(|| json!({})),
            privileges: body.get("privileges").cloned().unwrap_or_else(|| json!({})),
        },
    ) {
        Ok(level) => Json(ApiResponse::ok(json!(level))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_badges(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_badges(
        &ctx,
        ListBadgesCommand {
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn create_badge(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Json(body): Json<Value>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().create_badge(
        &ctx,
        CreateBadgeCommand {
            code: body
                .get("code")
                .or_else(|| body.get("slug"))
                .and_then(Value::as_str)
                .unwrap_or("badge")
                .to_string(),
            name: body.get("name").and_then(Value::as_str).unwrap_or("Badge").to_string(),
            description: body.get("description").and_then(Value::as_str).map(str::to_string),
            grant_mode: body
                .get("grantMode")
                .or_else(|| body.get("grant_mode"))
                .and_then(Value::as_str)
                .unwrap_or("manual")
                .to_string(),
            icon_media_id: body
                .get("iconMediaId")
                .or_else(|| body.get("icon_media_id"))
                .and_then(Value::as_str)
                .map(str::to_string),
            rule_json: body.get("ruleJson").or_else(|| body.get("rule_json")).cloned(),
        },
    ) {
        Ok(badge) => Json(ApiResponse::ok(json!(badge))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_board_stats(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_board_stats(
        &ctx,
        ListBoardStatsCommand {
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_topic_stats(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_topic_stats(
        &ctx,
        ListTopicStatsCommand {
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn reindex_search(State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx) -> Json<ApiResponse<Value>> {
    match state.service_host.service().rebuild_search_projection(
        &ctx,
        RebuildSearchProjectionCommand {
            scope: None,
            board_id: None,
        },
    ) {
        Ok(result) => Json(ApiResponse::ok(json!(result))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}

async fn list_audit_actions(
    State(state): State<AppState>, PromptsCtx(ctx): PromptsCtx,
    Query(query): Query<CursorQuery>,
) -> Json<ApiResponse<Value>> {
    match state.service_host.service().list_audit_actions(
        &ctx,
        ListAuditActionsCommand {
            cursor: query.cursor,
            limit: query.limit.unwrap_or(20),
        },
    ) {
        Ok(page) => Json(ApiResponse::ok(page_json(&page))),
        Err(error) => Json(ApiResponse::err(error.to_string())),
    }
}
