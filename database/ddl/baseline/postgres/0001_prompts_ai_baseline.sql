-- SDKWork Prompts baseline schema (intelligence/prompts, ai_ module prefix)
-- Authoritative contract: specs/prompts-ai-database.schema.yaml

CREATE TABLE IF NOT EXISTS ai_prompt_category (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    code VARCHAR(128) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_prompt_category_key
    ON ai_prompt_category (tenant_id, organization_id, code);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_category_tenant_status
    ON ai_prompt_category (tenant_id, organization_id, status, sort_order, id);

CREATE TABLE IF NOT EXISTS ai_prompt (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    prompt_key VARCHAR(128) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category_id BIGINT,
    category_code VARCHAR(128),
    prompt_type VARCHAR(64) NOT NULL,
    visibility VARCHAR(64) NOT NULL,
    owner_user_id BIGINT,
    latest_version_id BIGINT,
    published_version_id BIGINT,
    tags JSONB NOT NULL DEFAULT '[]'::jsonb,
    published_at TIMESTAMPTZ,
    deprecated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_prompt_key
    ON ai_prompt (tenant_id, organization_id, prompt_key);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_category
    ON ai_prompt (tenant_id, organization_id, category_id, status, updated_at, id);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_type
    ON ai_prompt (tenant_id, organization_id, prompt_type, status, updated_at, id);

CREATE TABLE IF NOT EXISTS ai_prompt_version (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    prompt_id BIGINT NOT NULL,
    version_no VARCHAR(64) NOT NULL,
    title VARCHAR(255),
    content TEXT NOT NULL,
    variable_schema JSONB NOT NULL DEFAULT '{}'::jsonb,
    output_schema JSONB NOT NULL DEFAULT '{}'::jsonb,
    model_constraints JSONB NOT NULL DEFAULT '{}'::jsonb,
    safety_policy JSONB NOT NULL DEFAULT '{}'::jsonb,
    examples JSONB NOT NULL DEFAULT '[]'::jsonb,
    checksum_hash VARCHAR(128),
    lifecycle_status VARCHAR(64) NOT NULL,
    review_status VARCHAR(64),
    review_comment TEXT,
    created_by BIGINT,
    published_at TIMESTAMPTZ,
    deprecated_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_prompt_version_no
    ON ai_prompt_version (tenant_id, organization_id, prompt_id, version_no);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_version_prompt
    ON ai_prompt_version (tenant_id, organization_id, prompt_id, lifecycle_status, created_at, id);

CREATE TABLE IF NOT EXISTS ai_prompt_binding (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version BIGINT NOT NULL DEFAULT 0,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    prompt_id BIGINT NOT NULL,
    prompt_version_id BIGINT,
    owner_type VARCHAR(64) NOT NULL,
    owner_id BIGINT NOT NULL,
    binding_role VARCHAR(64) NOT NULL,
    priority INTEGER NOT NULL DEFAULT 0,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    policy JSONB NOT NULL DEFAULT '{}'::jsonb,
    snapshot JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_ai_prompt_binding_owner
    ON ai_prompt_binding (tenant_id, organization_id, owner_type, owner_id, binding_role, priority, id);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_binding_prompt
    ON ai_prompt_binding (tenant_id, organization_id, prompt_id, prompt_version_id, enabled, id);

CREATE TABLE IF NOT EXISTS ai_prompt_template (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(96) NOT NULL,
    tenant_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL DEFAULT 0,
    owner_user_id BIGINT NOT NULL,
    template_key VARCHAR(128) NOT NULL,
    code VARCHAR(128) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    prompt_kind VARCHAR(32) NOT NULL,
    template_format VARCHAR(32) NOT NULL,
    template_body TEXT NOT NULL,
    variables_schema JSONB NOT NULL,
    model_constraints JSONB NOT NULL,
    capability_ids JSONB NOT NULL DEFAULT '[]'::jsonb,
    categories JSONB NOT NULL DEFAULT '[]'::jsonb,
    tags JSONB NOT NULL DEFAULT '[]'::jsonb,
    safety_profile_id VARCHAR(128),
    data_scope INTEGER NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    visibility INTEGER NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    deleted_by BIGINT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    CONSTRAINT ck_ai_prompt_template_key CHECK (
        template_key ~ '^prompt\.[a-z0-9_-]+(\.[a-z0-9_-]+)*$'
    ),
    CONSTRAINT ck_ai_prompt_template_prompt_kind CHECK (
        prompt_kind IN ('system', 'developer', 'user', 'workflow', 'tool', 'mcp-prompt')
    ),
    CONSTRAINT ck_ai_prompt_template_format CHECK (
        template_format IN ('plain-text', 'handlebars', 'liquid', 'jinja', 'json-schema')
    )
);

CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_prompt_template_uuid
    ON ai_prompt_template (uuid);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_prompt_template_tenant_key
    ON ai_prompt_template (tenant_id, template_key);
CREATE UNIQUE INDEX IF NOT EXISTS uk_ai_prompt_template_tenant_code
    ON ai_prompt_template (tenant_id, code);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_template_tenant_org_status
    ON ai_prompt_template (tenant_id, organization_id, status, updated_at DESC, code ASC);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_template_visibility
    ON ai_prompt_template (tenant_id, visibility, status);

CREATE TABLE IF NOT EXISTS ai_prompt_usage (
    id BIGINT NOT NULL PRIMARY KEY,
    uuid VARCHAR(64) NOT NULL,
    tenant_id BIGINT NOT NULL DEFAULT 0,
    organization_id BIGINT NOT NULL DEFAULT 0,
    status INTEGER NOT NULL DEFAULT 1,
    version BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    prompt_id BIGINT,
    prompt_version_id BIGINT,
    prompt_template_id BIGINT,
    actor_id BIGINT,
    surface VARCHAR(64) NOT NULL,
    context JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX IF NOT EXISTS idx_ai_prompt_usage_tenant_created
    ON ai_prompt_usage (tenant_id, created_at, id);
CREATE INDEX IF NOT EXISTS idx_ai_prompt_usage_prompt
    ON ai_prompt_usage (tenant_id, prompt_id, created_at, id);
