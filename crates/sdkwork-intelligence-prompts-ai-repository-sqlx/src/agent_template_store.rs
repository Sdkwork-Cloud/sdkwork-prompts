use sqlx::{PgPool, Row};

use sdkwork_intelligence_prompts_ai_contract::{
    domain::{
        AgentPromptTemplateKind, AgentPromptTemplateRecord, PromptBindingRecord, PromptRecord,
        PromptVersionRecord,
    },
    ports::AgentPromptTemplateListQuery,
    PromptAiError, PromptAiResult,
};

pub(crate) async fn get_prompt(pool: &PgPool, tenant_id: i64, id: i64) -> PromptAiResult<PromptRecord> {
    let row = sqlx::query(
        r#"
        SELECT id, uuid, tenant_id, organization_id, prompt_key, name, description,
               prompt_type, visibility, latest_version_id, published_version_id, status
        FROM ai_prompt
        WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(tenant_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| PromptAiError::internal(e.to_string()))?;

    let Some(row) = row else {
        return Err(PromptAiError::not_found("prompt was not found"));
    };

    Ok(PromptRecord {
        id: row.get("id"),
        uuid: row.get("uuid"),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        prompt_key: row.get("prompt_key"),
        name: row.get("name"),
        description: row.try_get("description").ok(),
        prompt_type: row.get("prompt_type"),
        visibility: row.get("visibility"),
        latest_version_id: row.try_get("latest_version_id").ok(),
        published_version_id: row.try_get("published_version_id").ok(),
        status: row.get("status"),
    })
}

pub(crate) async fn get_prompt_version(
    pool: &PgPool,
    tenant_id: i64,
    id: i64,
) -> PromptAiResult<PromptVersionRecord> {
    let row = sqlx::query(
        r#"
        SELECT id, uuid, tenant_id, organization_id, prompt_id, version_no, title, content, lifecycle_status
        FROM ai_prompt_version
        WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(tenant_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| PromptAiError::internal(e.to_string()))?;

    let Some(row) = row else {
        return Err(PromptAiError::not_found("prompt version was not found"));
    };

    Ok(PromptVersionRecord {
        id: row.get("id"),
        uuid: row.get("uuid"),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        prompt_id: row.get("prompt_id"),
        version_no: row.get("version_no"),
        title: row.try_get("title").ok(),
        content: row.get("content"),
        lifecycle_status: row.get("lifecycle_status"),
    })
}

pub(crate) async fn list_bindings_for_owner(
    pool: &PgPool,
    tenant_id: i64,
    organization_id: i64,
    owner_type: &str,
    owner_id: i64,
) -> PromptAiResult<Vec<PromptBindingRecord>> {
    let rows = sqlx::query(
        r#"
        SELECT id, uuid, tenant_id, organization_id, prompt_id, prompt_version_id,
               owner_type, owner_id, binding_role, priority, enabled
        FROM ai_prompt_binding
        WHERE tenant_id = $1 AND organization_id = $2
          AND owner_type = $3 AND owner_id = $4
          AND enabled = TRUE AND deleted_at IS NULL
        ORDER BY priority ASC, id ASC
        "#,
    )
    .bind(tenant_id)
    .bind(organization_id)
    .bind(owner_type)
    .bind(owner_id)
    .fetch_all(pool)
    .await
    .map_err(|e| PromptAiError::internal(e.to_string()))?;

    Ok(rows
        .iter()
        .map(|row| PromptBindingRecord {
            id: row.get("id"),
            uuid: row.get("uuid"),
            tenant_id: row.get("tenant_id"),
            organization_id: row.get("organization_id"),
            prompt_id: row.get("prompt_id"),
            prompt_version_id: row.try_get("prompt_version_id").ok(),
            owner_type: row.get("owner_type"),
            owner_id: row.get("owner_id"),
            binding_role: row.get("binding_role"),
            priority: row.get("priority"),
            enabled: row.get("enabled"),
        })
        .collect())
}

pub(crate) async fn get_agent_prompt_template(
    pool: &PgPool,
    tenant_id: i64,
    id: i64,
) -> PromptAiResult<AgentPromptTemplateRecord> {
    let row = sqlx::query(
        r#"
        SELECT id, uuid, tenant_id, organization_id, owner_user_id, prompt_id, code,
               display_name, description, prompt_kind, template_format, template_body,
               safety_profile_id, status, visibility
        FROM ai_agent_prompt_template
        WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(tenant_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| PromptAiError::internal(e.to_string()))?;

    let Some(row) = row else {
        return Err(PromptAiError::not_found("agent prompt template was not found"));
    };

    let prompt_kind: String = row.get("prompt_kind");
    let kind = match prompt_kind.as_str() {
        "system" => AgentPromptTemplateKind::System,
        "developer" => AgentPromptTemplateKind::Developer,
        "user" => AgentPromptTemplateKind::User,
        "workflow" => AgentPromptTemplateKind::Workflow,
        "tool" => AgentPromptTemplateKind::Tool,
        "mcp-prompt" => AgentPromptTemplateKind::McpPrompt,
        other => return Err(PromptAiError::internal(format!("unknown prompt_kind: {other}"))),
    };

    Ok(AgentPromptTemplateRecord {
        id: row.get("id"),
        uuid: row.get("uuid"),
        tenant_id: row.get("tenant_id"),
        organization_id: row.get("organization_id"),
        owner_user_id: row.get("owner_user_id"),
        prompt_id: row.get("prompt_id"),
        code: row.get("code"),
        display_name: row.get("display_name"),
        description: row.try_get("description").ok(),
        prompt_kind: kind,
        template_format: row.get("template_format"),
        template_body: row.get("template_body"),
        safety_profile_id: row.try_get("safety_profile_id").ok(),
        status: row.get("status"),
        visibility: row.get("visibility"),
    })
}

pub(crate) async fn list_agent_prompt_templates(
    pool: &PgPool,
    query: AgentPromptTemplateListQuery,
) -> PromptAiResult<Vec<AgentPromptTemplateRecord>> {
    let limit = query.limit.clamp(1, 200) as i64;
    let rows = sqlx::query(
        r#"
        SELECT id FROM ai_agent_prompt_template
        WHERE tenant_id = $1 AND organization_id = $2 AND deleted_at IS NULL
        ORDER BY updated_at DESC, id DESC
        LIMIT $3
        "#,
    )
    .bind(query.tenant_id)
    .bind(query.organization_id)
    .bind(limit)
    .fetch_all(pool)
    .await
    .map_err(|e| PromptAiError::internal(e.to_string()))?;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let id: i64 = row.get("id");
        items.push(get_agent_prompt_template(pool, query.tenant_id, id).await?);
    }
    Ok(items)
}
