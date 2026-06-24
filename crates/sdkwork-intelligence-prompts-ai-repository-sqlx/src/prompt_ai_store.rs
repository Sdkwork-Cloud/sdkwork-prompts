use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};

use sdkwork_intelligence_prompts_ai_contract::{
    commands::{
        CreatePromptBindingCommand, CreatePromptCommand, CreatePromptVersionCommand,
        ListPromptBindingsQuery, ListPromptsQuery, ListPromptVersionsQuery, PromptAiBindingItem,
        PromptAiItem, PromptAiSubject, PromptAiVersionItem, PublishPromptVersionCommand,
        RenderPromptVersionCommand, UpdatePromptBindingCommand, UpdatePromptCommand,
    },
    domain::{AgentPromptTemplateRecord, PromptBindingRecord, PromptRecord, PromptVersionRecord},
    ports::{AgentPromptTemplateListQuery, PromptAiRepository},
    PromptAiError, PromptAiResult,
};
use crate::stable_id::stable_uuid;
use crate::SqlxPromptAiRepository;

use async_trait::async_trait;
async fn list_prompts(
    pool: &PgPool,
    query: ListPromptsQuery,
) -> PromptAiResult<Vec<PromptAiItem>> {
    let status = status_code(query.status.as_deref())?;
    let (category_id, category_code) = category_filter(query.category_id.as_deref())?;
    let rows = sqlx::query(
        r#"
        SELECT
            p.id,
            p.uuid,
            p.tenant_id,
            p.organization_id,
            p.prompt_key,
            p.name,
            p.description,
            p.category_id,
            p.category_code,
            p.prompt_type,
            p.visibility,
            p.owner_user_id,
            p.latest_version_id,
            p.published_version_id,
            CASE p.status WHEN 1 THEN 'enabled' WHEN 0 THEN 'disabled' ELSE CAST(p.status AS TEXT) END AS status,
            p.tags,
            CAST(p.created_at AS TEXT) AS created_at,
            CAST(p.updated_at AS TEXT) AS updated_at
        FROM ai_prompt p
        WHERE p.tenant_id = $1
          AND p.organization_id = $2
          AND p.deleted_at IS NULL
          AND ($3::text IS NULL OR lower(p.prompt_key) LIKE $3 OR lower(p.name) LIKE $3 OR lower(COALESCE(p.description, '')) LIKE $3)
          AND ($4::text IS NULL OR p.prompt_type = $4)
          AND ($5::text IS NULL OR p.visibility = $5)
          AND ($6::integer IS NULL OR p.status = $6)
          AND ($7::bigint IS NULL OR p.category_id = $7)
          AND ($8::text IS NULL OR p.category_code = $8)
        ORDER BY p.updated_at DESC NULLS LAST, p.id DESC
        LIMIT $9 OFFSET $10
        "#,
    )
    .bind(query.subject.tenant_id)
    .bind(query.subject.organization_id)
    .bind(like_filter(query.keyword.as_deref()))
    .bind(query.prompt_type.as_deref())
    .bind(query.visibility.as_deref())
    .bind(status)
    .bind(category_id)
    .bind(category_code.as_deref())
    .bind(query.page_size)
    .bind(query.offset)
    .fetch_all(pool)
    .await
    .map_err(store_error)?;

    rows.iter().map(row_to_prompt).collect()
}

async fn update_prompt(
    pool: &PgPool,
    command: UpdatePromptCommand,
) -> PromptAiResult<PromptAiItem> {
    let current = load_prompt_optional(pool, command.subject, command.prompt_id)
        .await?
        .ok_or_else(|| PromptAiError::not_found("prompt was not found"))?;
    let name = command.name.unwrap_or(current.name);
    let description = command.description.or(current.description);
    let tags = command.tags.unwrap_or(current.tags);
    let status = match command.status.as_deref() {
        Some(value) => app_template_status_code(Some(value))?.unwrap_or(1),
        None => status_code(Some(current.status.as_str()))?.unwrap_or(1),
    };
    let tags_json = json_text(&serde_json::Value::Array(
        tags.iter()
            .cloned()
            .map(serde_json::Value::String)
            .collect(),
    ));
    sqlx::query(
        r#"
        UPDATE ai_prompt
        SET name = $1,
            description = $2,
            tags = $3::jsonb,
            status = $4,
            updated_at = now()
        WHERE tenant_id = $5
          AND organization_id = $6
          AND id = $7
          AND deleted_at IS NULL
        "#,
    )
    .bind(&name)
    .bind(&description)
    .bind(tags_json)
    .bind(status)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.prompt_id)
    .execute(pool)
    .await
    .map_err(|error| write_error("failed to update prompt", error))?;
    load_prompt(pool, command.subject, command.prompt_id).await
}

async fn create_prompt(
    pool: &PgPool,
    command: CreatePromptCommand,
) -> PromptAiResult<PromptAiItem> {
    let (category_id, category_code) = category_filter(command.category_id.as_deref())?;
    let tags = json_text(&serde_json::Value::Array(
        command
            .tags
            .iter()
            .cloned()
            .map(serde_json::Value::String)
            .collect(),
    ));
    let id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO ai_prompt
            (uuid, tenant_id, organization_id, status, prompt_key, name, description,
             category_id, category_code, prompt_type, visibility, owner_user_id, tags)
        VALUES
            ($1, $2, $3, 1, $4, $5, $6, $7, $8, $9, $10, $11, $12::jsonb)
        RETURNING id
        "#,
    )
    .bind(stable_uuid(
        "ai-prompt",
        &[
            &command.subject.tenant_id.to_string(),
            &command.subject.organization_id.to_string(),
            &command.prompt_key,
        ],
    ))
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(&command.prompt_key)
    .bind(&command.name)
    .bind(command.description.as_deref())
    .bind(category_id)
    .bind(category_code.as_deref())
    .bind(&command.prompt_type)
    .bind(&command.visibility)
    .bind(command.subject.operator_id)
    .bind(&tags)
    .fetch_one(pool)
    .await
    .map_err(|error| write_error("failed to create prompt", error))?;

    load_prompt(pool, command.subject, id).await
}

async fn list_versions(
    pool: &PgPool,
    query: ListPromptVersionsQuery,
) -> PromptAiResult<Vec<PromptAiVersionItem>> {
    ensure_prompt_exists(pool, query.subject, query.prompt_id).await?;
    let rows = sqlx::query(
        r#"
        SELECT
            id, uuid, tenant_id, organization_id, prompt_id, version_no,
            COALESCE(title, '') AS title,
            content,
            variable_schema,
            output_schema,
            model_constraints,
            safety_policy,
            examples_json,
            COALESCE(checksum_hash, '') AS checksum_hash,
            lifecycle_status,
            COALESCE(review_status, '') AS review_status,
            review_comment,
            COALESCE(created_by, 0) AS created_by,
            CAST(published_at AS TEXT) AS published_at,
            CAST(created_at AS TEXT) AS created_at,
            CAST(updated_at AS TEXT) AS updated_at
        FROM ai_prompt_version
        WHERE tenant_id = $1
          AND organization_id = $2
          AND prompt_id = $3
          AND deleted_at IS NULL
        ORDER BY created_at DESC NULLS LAST, id DESC
        "#,
    )
    .bind(query.subject.tenant_id)
    .bind(query.subject.organization_id)
    .bind(query.prompt_id)
    .fetch_all(pool)
    .await
    .map_err(store_error)?;

    rows.iter().map(row_to_prompt_version).collect()
}

async fn create_version(
    pool: &PgPool,
    command: CreatePromptVersionCommand,
) -> PromptAiResult<PromptAiVersionItem> {
    ensure_prompt_exists(pool, command.subject, command.prompt_id).await?;
    let checksum = checksum_hash(&[
        &command.content,
        &json_text(&command.variable_schema),
        &json_text(&command.output_schema),
        &json_text(&command.model_constraints),
        &json_text(&command.safety_policy),
        &json_text(&command.examples_json),
    ]);
    let id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO ai_prompt_version
            (uuid, tenant_id, organization_id, status, prompt_id, version_no, title, content,
             variable_schema, output_schema, model_constraints, safety_policy, examples_json,
             checksum_hash, lifecycle_status, review_status, created_by)
        VALUES
            ($1, $2, $3, 1, $4, $5, $6, $7, $8::jsonb, $9::jsonb, $10::jsonb, $11::jsonb, $12::jsonb, $13, 'draft', 'pending', $14)
        RETURNING id
        "#,
    )
    .bind(stable_uuid(
        "ai-prompt-version",
        &[
            &command.subject.tenant_id.to_string(),
            &command.subject.organization_id.to_string(),
            &command.prompt_id.to_string(),
            &command.version_no,
        ],
    ))
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.prompt_id)
    .bind(&command.version_no)
    .bind(&command.title)
    .bind(&command.content)
    .bind(json_text(&command.variable_schema))
    .bind(json_text(&command.output_schema))
    .bind(json_text(&command.model_constraints))
    .bind(json_text(&command.safety_policy))
    .bind(json_text(&command.examples_json))
    .bind(&checksum)
    .bind(command.subject.operator_id)
    .fetch_one(pool)
    .await
    .map_err(|error| write_error("failed to create prompt version", error))?;

    sqlx::query(
        r#"
        UPDATE ai_prompt
        SET latest_version_id = $1, updated_at = now()
        WHERE tenant_id = $2 AND organization_id = $3 AND id = $4
        "#,
    )
    .bind(id)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.prompt_id)
    .execute(pool)
    .await
    .map_err(store_error)?;

    load_version(pool, command.subject, id).await
}

async fn publish_version(
    pool: &PgPool,
    command: PublishPromptVersionCommand,
) -> PromptAiResult<Option<PromptAiVersionItem>> {
    let Some(version) = load_version_optional(pool, command.subject, command.version_id).await?
    else {
        return Ok(None);
    };
    sqlx::query(
        r#"
        UPDATE ai_prompt_version
        SET lifecycle_status = 'published',
            review_status = 'approved',
            published_at = now(),
            updated_at = now()
        WHERE tenant_id = $1 AND organization_id = $2 AND id = $3
        "#,
    )
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.version_id)
    .execute(pool)
    .await
    .map_err(store_error)?;

    sqlx::query(
        r#"
        UPDATE ai_prompt
        SET published_version_id = $1,
            latest_version_id = COALESCE(latest_version_id, $1),
            published_at = now(),
            updated_at = now()
        WHERE tenant_id = $2 AND organization_id = $3 AND id = $4
        "#,
    )
    .bind(command.version_id)
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(version.prompt_id)
    .execute(pool)
    .await
    .map_err(store_error)?;

    load_version_optional(pool, command.subject, command.version_id).await
}

async fn render_version(
    pool: &PgPool,
    command: RenderPromptVersionCommand,
) -> PromptAiResult<Option<String>> {
    let Some(version) = load_version_optional(pool, command.subject, command.version_id).await?
    else {
        return Ok(None);
    };
    let mut rendered = version.content;
    let Some(variables) = command.variables.as_object() else {
        return Ok(Some(rendered));
    };
    for (key, value) in variables {
        let value = match value {
            serde_json::Value::String(value) => value.clone(),
            serde_json::Value::Null => String::new(),
            value => value.to_string(),
        };
        rendered = rendered.replace(&format!("{{{{{key}}}}}"), &value);
        rendered = rendered.replace(&format!("{{{{ {key} }}}}"), &value);
    }
    Ok(Some(rendered))
}

async fn list_bindings(
    pool: &PgPool,
    query: ListPromptBindingsQuery,
) -> PromptAiResult<Vec<PromptAiBindingItem>> {
    ensure_prompt_exists(pool, query.subject, query.prompt_id).await?;
    let rows = sqlx::query(
        r#"
        SELECT
            id, uuid, tenant_id, organization_id, prompt_id, prompt_version_id,
            owner_type, owner_id, binding_role, priority, enabled,
            policy_json, snapshot_json,
            CAST(created_at AS TEXT) AS created_at,
            CAST(updated_at AS TEXT) AS updated_at
        FROM ai_prompt_binding
        WHERE tenant_id = $1
          AND organization_id = $2
          AND prompt_id = $3
          AND deleted_at IS NULL
        ORDER BY priority ASC, id ASC
        "#,
    )
    .bind(query.subject.tenant_id)
    .bind(query.subject.organization_id)
    .bind(query.prompt_id)
    .fetch_all(pool)
    .await
    .map_err(store_error)?;

    rows.iter().map(row_to_prompt_binding).collect()
}

async fn create_binding(
    pool: &PgPool,
    command: CreatePromptBindingCommand,
) -> PromptAiResult<PromptAiBindingItem> {
    ensure_prompt_exists(pool, command.subject, command.prompt_id).await?;
    if let Some(version_id) = command.prompt_version_id {
        ensure_prompt_version_belongs(pool, command.subject, command.prompt_id, version_id).await?;
    }
    let snapshot = prompt_binding_snapshot(
        command.prompt_id,
        command.prompt_version_id,
        &command.owner_type,
        command.owner_id,
        &command.binding_role,
        command.priority,
        command.enabled,
        &command.policy_json,
    );
    let id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO ai_prompt_binding
            (uuid, tenant_id, organization_id, status, prompt_id, prompt_version_id,
             owner_type, owner_id, binding_role, priority, enabled, policy_json, snapshot_json)
        VALUES
            ($1, $2, $3, 1, $4, $5, $6, $7, $8, $9, $10, $11::jsonb, $12::jsonb)
        RETURNING id
        "#,
    )
    .bind(stable_uuid(
        "ai-prompt-binding",
        &[
            &command.subject.tenant_id.to_string(),
            &command.subject.organization_id.to_string(),
            &command.prompt_id.to_string(),
            &command
                .prompt_version_id
                .map(|value| value.to_string())
                .unwrap_or_default(),
            &command.owner_type,
            &command.owner_id.to_string(),
            &command.binding_role,
        ],
    ))
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.prompt_id)
    .bind(command.prompt_version_id)
    .bind(&command.owner_type)
    .bind(command.owner_id)
    .bind(&command.binding_role)
    .bind(command.priority)
    .bind(command.enabled)
    .bind(json_text(&command.policy_json))
    .bind(json_text(&snapshot))
    .fetch_one(pool)
    .await
    .map_err(|error| write_error("failed to create prompt binding", error))?;

    load_binding(pool, command.subject, id).await
}

async fn update_binding(
    pool: &PgPool,
    command: UpdatePromptBindingCommand,
) -> PromptAiResult<Option<PromptAiBindingItem>> {
    let Some(current) = load_binding_optional(pool, command.subject, command.binding_id).await?
    else {
        return Ok(None);
    };
    let prompt_version_id = command
        .prompt_version_id
        .unwrap_or(current.prompt_version_id);
    if let Some(version_id) = prompt_version_id {
        ensure_prompt_version_belongs(pool, command.subject, current.prompt_id, version_id).await?;
    }
    let owner_type = command.owner_type.unwrap_or(current.owner_type);
    let owner_id = command.owner_id.unwrap_or(current.owner_id);
    let binding_role = command.binding_role.unwrap_or(current.binding_role);
    let priority = command.priority.unwrap_or(current.priority);
    let enabled = command.enabled.unwrap_or(current.enabled);
    let policy_json = command.policy_json.unwrap_or(current.policy_json);
    let snapshot = prompt_binding_snapshot(
        current.prompt_id,
        prompt_version_id,
        &owner_type,
        owner_id,
        &binding_role,
        priority,
        enabled,
        &policy_json,
    );
    sqlx::query(
        r#"
        UPDATE ai_prompt_binding
        SET prompt_version_id = $1,
            owner_type = $2,
            owner_id = $3,
            binding_role = $4,
            priority = $5,
            enabled = $6,
            policy_json = $7::jsonb,
            snapshot_json = $8::jsonb,
            updated_at = now()
        WHERE tenant_id = $9
          AND organization_id = $10
          AND id = $11
          AND deleted_at IS NULL
        "#,
    )
    .bind(prompt_version_id)
    .bind(&owner_type)
    .bind(owner_id)
    .bind(&binding_role)
    .bind(priority)
    .bind(enabled)
    .bind(json_text(&policy_json))
    .bind(json_text(&snapshot))
    .bind(command.subject.tenant_id)
    .bind(command.subject.organization_id)
    .bind(command.binding_id)
    .execute(pool)
    .await
    .map_err(|error| write_error("failed to update prompt binding", error))?;

    load_binding_optional(pool, command.subject, command.binding_id).await
}

async fn load_prompt(
    pool: &PgPool,
    subject: PromptAiSubject,
    id: i64,
) -> PromptAiResult<PromptAiItem> {
    load_prompt_optional(pool, subject, id)
        .await?
        .ok_or_else(|| PromptAiError::not_found("prompt was not found"))
}

async fn load_prompt_optional(
    pool: &PgPool,
    subject: PromptAiSubject,
    id: i64,
) -> PromptAiResult<Option<PromptAiItem>> {
    let row = sqlx::query(
        r#"
        SELECT
            p.id,
            p.uuid,
            p.tenant_id,
            p.organization_id,
            p.prompt_key,
            p.name,
            p.description,
            p.category_id,
            p.category_code,
            p.prompt_type,
            p.visibility,
            p.owner_user_id,
            p.latest_version_id,
            p.published_version_id,
            CASE p.status WHEN 1 THEN 'enabled' WHEN 0 THEN 'disabled' ELSE CAST(p.status AS TEXT) END AS status,
            p.tags,
            CAST(p.created_at AS TEXT) AS created_at,
            CAST(p.updated_at AS TEXT) AS updated_at
        FROM ai_prompt p
        WHERE p.tenant_id = $1
          AND p.organization_id = $2
          AND p.id = $3
          AND p.deleted_at IS NULL
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(store_error)?;
    row.as_ref().map(row_to_prompt).transpose()
}

async fn load_version(
    pool: &PgPool,
    subject: PromptAiSubject,
    id: i64,
) -> PromptAiResult<PromptAiVersionItem> {
    load_version_optional(pool, subject, id)
        .await?
        .ok_or_else(|| PromptAiError::not_found("prompt version was not found"))
}

async fn load_version_optional(
    pool: &PgPool,
    subject: PromptAiSubject,
    id: i64,
) -> PromptAiResult<Option<PromptAiVersionItem>> {
    let row = sqlx::query(
        r#"
        SELECT
            id, uuid, tenant_id, organization_id, prompt_id, version_no,
            COALESCE(title, '') AS title,
            content,
            variable_schema,
            output_schema,
            model_constraints,
            safety_policy,
            examples_json,
            COALESCE(checksum_hash, '') AS checksum_hash,
            lifecycle_status,
            COALESCE(review_status, '') AS review_status,
            review_comment,
            COALESCE(created_by, 0) AS created_by,
            CAST(published_at AS TEXT) AS published_at,
            CAST(created_at AS TEXT) AS created_at,
            CAST(updated_at AS TEXT) AS updated_at
        FROM ai_prompt_version
        WHERE tenant_id = $1
          AND organization_id = $2
          AND id = $3
          AND deleted_at IS NULL
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(store_error)?;
    row.as_ref().map(row_to_prompt_version).transpose()
}

async fn load_binding(
    pool: &PgPool,
    subject: PromptAiSubject,
    id: i64,
) -> PromptAiResult<PromptAiBindingItem> {
    load_binding_optional(pool, subject, id)
        .await?
        .ok_or_else(|| PromptAiError::not_found("prompt binding was not found"))
}

async fn load_binding_optional(
    pool: &PgPool,
    subject: PromptAiSubject,
    id: i64,
) -> PromptAiResult<Option<PromptAiBindingItem>> {
    let row = sqlx::query(
        r#"
        SELECT
            id, uuid, tenant_id, organization_id, prompt_id, prompt_version_id,
            owner_type, owner_id, binding_role, priority, enabled,
            policy_json, snapshot_json,
            CAST(created_at AS TEXT) AS created_at,
            CAST(updated_at AS TEXT) AS updated_at
        FROM ai_prompt_binding
        WHERE tenant_id = $1
          AND organization_id = $2
          AND id = $3
          AND deleted_at IS NULL
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(store_error)?;
    row.as_ref().map(row_to_prompt_binding).transpose()
}

async fn ensure_prompt_exists(
    pool: &PgPool,
    subject: PromptAiSubject,
    prompt_id: i64,
) -> PromptAiResult<()> {
    let exists: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT 1
        FROM ai_prompt
        WHERE tenant_id = $1
          AND organization_id = $2
          AND id = $3
          AND deleted_at IS NULL
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(prompt_id)
    .fetch_optional(pool)
    .await
    .map_err(store_error)?;
    if exists.is_none() {
        return Err(PromptAiError::not_found("prompt was not found"));
    }
    Ok(())
}

async fn ensure_prompt_version_belongs(
    pool: &PgPool,
    subject: PromptAiSubject,
    prompt_id: i64,
    version_id: i64,
) -> PromptAiResult<()> {
    let exists: Option<i64> = sqlx::query_scalar(
        r#"
        SELECT 1
        FROM ai_prompt_version
        WHERE tenant_id = $1
          AND organization_id = $2
          AND prompt_id = $3
          AND id = $4
          AND deleted_at IS NULL
        LIMIT 1
        "#,
    )
    .bind(subject.tenant_id)
    .bind(subject.organization_id)
    .bind(prompt_id)
    .bind(version_id)
    .fetch_optional(pool)
    .await
    .map_err(store_error)?;
    if exists.is_none() {
        return Err(PromptAiError::not_found(
            "prompt version was not found for prompt",
        ));
    }
    Ok(())
}

fn row_to_prompt(row: &sqlx::postgres::PgRow) -> PromptAiResult<PromptAiItem> {
    Ok(PromptAiItem {
        id: integer_cell(row, "id")?,
        uuid: string_cell(row, "uuid")?,
        tenant_id: integer_cell(row, "tenant_id")?,
        organization_id: integer_cell(row, "organization_id")?,
        prompt_key: string_cell(row, "prompt_key")?,
        name: string_cell(row, "name")?,
        description: optional_string_cell(row, "description")?,
        category_id: optional_integer_cell(row, "category_id")?.map(|value| value.to_string()),
        category_code: optional_string_cell(row, "category_code")?,
        prompt_type: string_cell(row, "prompt_type")?,
        visibility: string_cell(row, "visibility")?,
        owner_user_id: optional_integer_cell(row, "owner_user_id")?,
        latest_version_id: optional_integer_cell(row, "latest_version_id")?,
        published_version_id: optional_integer_cell(row, "published_version_id")?,
        status: string_cell(row, "status")?,
        tags: tags_cell(row, "tags")?,
        created_at: string_cell(row, "created_at")?,
        updated_at: string_cell(row, "updated_at")?,
    })
}

fn row_to_prompt_version(row: &sqlx::postgres::PgRow) -> PromptAiResult<PromptAiVersionItem> {
    Ok(PromptAiVersionItem {
        id: integer_cell(row, "id")?,
        uuid: string_cell(row, "uuid")?,
        tenant_id: integer_cell(row, "tenant_id")?,
        organization_id: integer_cell(row, "organization_id")?,
        prompt_id: integer_cell(row, "prompt_id")?,
        version_no: string_cell(row, "version_no")?,
        title: string_cell(row, "title")?,
        content: string_cell(row, "content")?,
        variable_schema: json_cell(row, "variable_schema")?,
        output_schema: json_cell(row, "output_schema")?,
        model_constraints: json_cell(row, "model_constraints")?,
        safety_policy: json_cell(row, "safety_policy")?,
        examples_json: json_cell(row, "examples_json")?,
        checksum_hash: string_cell(row, "checksum_hash")?,
        lifecycle_status: string_cell(row, "lifecycle_status")?,
        review_status: string_cell(row, "review_status")?,
        review_comment: optional_string_cell(row, "review_comment")?,
        created_by: integer_cell(row, "created_by")?,
        published_at: optional_string_cell(row, "published_at")?,
        created_at: string_cell(row, "created_at")?,
        updated_at: string_cell(row, "updated_at")?,
    })
}

fn row_to_prompt_binding(row: &sqlx::postgres::PgRow) -> PromptAiResult<PromptAiBindingItem> {
    Ok(PromptAiBindingItem {
        id: integer_cell(row, "id")?,
        uuid: string_cell(row, "uuid")?,
        tenant_id: integer_cell(row, "tenant_id")?,
        organization_id: integer_cell(row, "organization_id")?,
        prompt_id: integer_cell(row, "prompt_id")?,
        prompt_version_id: optional_integer_cell(row, "prompt_version_id")?,
        owner_type: string_cell(row, "owner_type")?,
        owner_id: integer_cell(row, "owner_id")?,
        binding_role: string_cell(row, "binding_role")?,
        priority: integer_cell(row, "priority")? as i32,
        enabled: bool_cell(row, "enabled")?,
        policy_json: json_cell(row, "policy_json")?,
        snapshot_json: json_cell(row, "snapshot_json")?,
        created_at: string_cell(row, "created_at")?,
        updated_at: string_cell(row, "updated_at")?,
    })
}

fn like_filter(value: Option<&str>) -> Option<String> {
    value.map(|value| format!("%{}%", value.to_ascii_lowercase()))
}

fn category_filter(value: Option<&str>) -> PromptAiResult<(Option<i64>, Option<String>)> {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok((None, None));
    };
    if let Ok(id) = value.parse::<i64>() {
        if id <= 0 {
            return Err(PromptAiError::validation("prompt categoryId must be positive"));
        }
        return Ok((Some(id), None));
    }
    Ok((None, Some(value.to_owned())))
}

fn status_code(status: Option<&str>) -> PromptAiResult<Option<i32>> {
    match status {
        None => Ok(None),
        Some("enabled") | Some("active") => Ok(Some(1)),
        Some("disabled") | Some("archived") | Some("draft") => Ok(Some(0)),
        Some(value) if value.parse::<i32>().is_ok() => Ok(value.parse().ok()),
        Some(value) => Err(PromptAiError::validation(format!(
            "unsupported prompt status: {value}"
        ))),
    }
}

fn app_template_status_code(status: Option<&str>) -> PromptAiResult<Option<i32>> {
    status_code(status)
}

fn checksum_hash(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0]);
    }
    format!("{:x}", hasher.finalize())
}

fn prompt_binding_snapshot(
    prompt_id: i64,
    prompt_version_id: Option<i64>,
    owner_type: &str,
    owner_id: i64,
    binding_role: &str,
    priority: i32,
    enabled: bool,
    policy_json: &serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "promptId": prompt_id,
        "promptVersionId": prompt_version_id,
        "ownerType": owner_type,
        "ownerId": owner_id,
        "bindingRole": binding_role,
        "priority": priority,
        "enabled": enabled,
        "policyJson": policy_json,
    })
}

fn json_text(value: &serde_json::Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "{}".to_owned())
}

fn tags_cell(row: &sqlx::postgres::PgRow, column: &str) -> PromptAiResult<Vec<String>> {
    let value = json_cell(row, column)?;
    Ok(value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default())
}

fn json_cell(row: &sqlx::postgres::PgRow, column: &str) -> PromptAiResult<serde_json::Value> {
    if let Ok(value) = row.try_get::<Option<serde_json::Value>, _>(column) {
        return Ok(value.unwrap_or_else(|| serde_json::json!({})));
    }
    if let Ok(value) = row.try_get::<serde_json::Value, _>(column) {
        return Ok(value);
    }
    let raw = string_cell(row, column)?;
    if raw.trim().is_empty() {
        return Ok(serde_json::json!({}));
    }
    serde_json::from_str(&raw)
        .map_err(|error| PromptAiError::validation(format!("invalid prompt json {column}: {error}")))
}

fn string_cell(row: &sqlx::postgres::PgRow, column: &str) -> PromptAiResult<String> {
    if let Ok(value) = row.try_get::<Option<String>, _>(column) {
        return Ok(value.unwrap_or_default());
    }
    if let Ok(value) = row.try_get::<String, _>(column) {
        return Ok(value);
    }
    if let Ok(value) = row.try_get::<Option<i64>, _>(column) {
        return Ok(value.map(|value| value.to_string()).unwrap_or_default());
    }
    if let Ok(value) = row.try_get::<i64, _>(column) {
        return Ok(value.to_string());
    }
    if let Ok(value) = row.try_get::<Option<i32>, _>(column) {
        return Ok(value.map(|value| value.to_string()).unwrap_or_default());
    }
    if let Ok(value) = row.try_get::<i32, _>(column) {
        return Ok(value.to_string());
    }
    Err(PromptAiError::validation(format!(
        "prompt row column {column} is not readable as text"
    )))
}

fn optional_string_cell(row: &sqlx::postgres::PgRow, column: &str) -> PromptAiResult<Option<String>> {
    let value = string_cell(row, column)?;
    Ok((!value.trim().is_empty()).then_some(value))
}

fn integer_cell(row: &sqlx::postgres::PgRow, column: &str) -> PromptAiResult<i64> {
    if let Ok(value) = row.try_get::<i64, _>(column) {
        return Ok(value);
    }
    if let Ok(value) = row.try_get::<Option<i64>, _>(column) {
        return Ok(value.unwrap_or_default());
    }
    if let Ok(value) = row.try_get::<i32, _>(column) {
        return Ok(i64::from(value));
    }
    if let Ok(value) = row.try_get::<Option<i32>, _>(column) {
        return Ok(value.map(i64::from).unwrap_or_default());
    }
    let value = string_cell(row, column)?;
    if value.trim().is_empty() {
        return Ok(0);
    }
    value
        .parse::<i64>()
        .map_err(|error| PromptAiError::validation(format!("invalid prompt integer {column}: {error}")))
}

fn optional_integer_cell(row: &sqlx::postgres::PgRow, column: &str) -> PromptAiResult<Option<i64>> {
    if let Ok(value) = row.try_get::<Option<i64>, _>(column) {
        return Ok(value);
    }
    if let Ok(value) = row.try_get::<i64, _>(column) {
        return Ok(Some(value));
    }
    if let Ok(value) = row.try_get::<Option<i32>, _>(column) {
        return Ok(value.map(i64::from));
    }
    if let Ok(value) = row.try_get::<i32, _>(column) {
        return Ok(Some(i64::from(value)));
    }
    let value = string_cell(row, column)?;
    if value.trim().is_empty() {
        return Ok(None);
    }
    value
        .parse::<i64>()
        .map(Some)
        .map_err(|error| PromptAiError::validation(format!("invalid prompt integer {column}: {error}")))
}

fn bool_cell(row: &sqlx::postgres::PgRow, column: &str) -> PromptAiResult<bool> {
    if let Ok(value) = row.try_get::<bool, _>(column) {
        return Ok(value);
    }
    if let Ok(value) = row.try_get::<i64, _>(column) {
        return Ok(value != 0);
    }
    if let Ok(value) = row.try_get::<i32, _>(column) {
        return Ok(value != 0);
    }
    let value = string_cell(row, column)?.to_ascii_lowercase();
    Ok(matches!(value.as_str(), "1" | "true" | "t" | "yes"))
}

fn write_error(context: &str, error: sqlx::Error) -> PromptAiError {
    let message = error.to_string();
    if message.contains("duplicate key value") || message.contains("unique constraint") {
        return PromptAiError::conflict(format!("{context}: record already exists"));
    }
    PromptAiError::internal(format!("{context}: {message}"))
}

fn store_error(error: sqlx::Error) -> PromptAiError {
    PromptAiError::internal(error.to_string())
}

#[async_trait]
impl PromptAiRepository for SqlxPromptAiRepository {
    async fn list_prompts(&self, query: ListPromptsQuery) -> PromptAiResult<Vec<PromptAiItem>> {
        list_prompts(&self.pool, query).await
    }

    async fn create_prompt(&self, command: CreatePromptCommand) -> PromptAiResult<PromptAiItem> {
        create_prompt(&self.pool, command).await
    }

    async fn update_prompt(&self, command: UpdatePromptCommand) -> PromptAiResult<PromptAiItem> {
        update_prompt(&self.pool, command).await
    }

    async fn list_versions(
        &self,
        query: ListPromptVersionsQuery,
    ) -> PromptAiResult<Vec<PromptAiVersionItem>> {
        list_versions(&self.pool, query).await
    }

    async fn create_version(
        &self,
        command: CreatePromptVersionCommand,
    ) -> PromptAiResult<PromptAiVersionItem> {
        create_version(&self.pool, command).await
    }

    async fn publish_version(
        &self,
        command: PublishPromptVersionCommand,
    ) -> PromptAiResult<Option<PromptAiVersionItem>> {
        publish_version(&self.pool, command).await
    }

    async fn render_version(
        &self,
        command: RenderPromptVersionCommand,
    ) -> PromptAiResult<Option<String>> {
        render_version(&self.pool, command).await
    }

    async fn list_bindings(
        &self,
        query: ListPromptBindingsQuery,
    ) -> PromptAiResult<Vec<PromptAiBindingItem>> {
        list_bindings(&self.pool, query).await
    }

    async fn create_binding(
        &self,
        command: CreatePromptBindingCommand,
    ) -> PromptAiResult<PromptAiBindingItem> {
        create_binding(&self.pool, command).await
    }

    async fn update_binding(
        &self,
        command: UpdatePromptBindingCommand,
    ) -> PromptAiResult<Option<PromptAiBindingItem>> {
        update_binding(&self.pool, command).await
    }

    async fn get_prompt(&self, tenant_id: i64, id: i64) -> PromptAiResult<PromptRecord> {
        crate::agent_template_store::get_prompt(&self.pool, tenant_id, id).await
    }

    async fn get_prompt_version(
        &self,
        tenant_id: i64,
        id: i64,
    ) -> PromptAiResult<PromptVersionRecord> {
        crate::agent_template_store::get_prompt_version(&self.pool, tenant_id, id).await
    }

    async fn list_bindings_for_owner(
        &self,
        tenant_id: i64,
        organization_id: i64,
        owner_type: &str,
        owner_id: i64,
    ) -> PromptAiResult<Vec<PromptBindingRecord>> {
        crate::agent_template_store::list_bindings_for_owner(
            &self.pool,
            tenant_id,
            organization_id,
            owner_type,
            owner_id,
        )
        .await
    }

    async fn get_agent_prompt_template(
        &self,
        tenant_id: i64,
        id: i64,
    ) -> PromptAiResult<AgentPromptTemplateRecord> {
        crate::agent_template_store::get_agent_prompt_template(&self.pool, tenant_id, id).await
    }

    async fn list_agent_prompt_templates(
        &self,
        query: AgentPromptTemplateListQuery,
    ) -> PromptAiResult<Vec<AgentPromptTemplateRecord>> {
        crate::agent_template_store::list_agent_prompt_templates(&self.pool, query).await
    }
}
