-- SDKWork Prompts baseline schema (intelligence/prompts)

CREATE TABLE IF NOT EXISTS prm_category (
    id BIGSERIAL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    slug VARCHAR(128) NOT NULL,
    display_name VARCHAR(256) NOT NULL,
    description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, slug)
);

CREATE TABLE IF NOT EXISTS prm_template (
    id BIGSERIAL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    category_id BIGINT REFERENCES prm_category(id),
    key VARCHAR(128) NOT NULL,
    name VARCHAR(256) NOT NULL,
    description TEXT,
    tags JSONB NOT NULL DEFAULT '[]'::jsonb,
    status VARCHAR(32) NOT NULL DEFAULT 'draft',
    latest_version_id BIGINT,
    created_by BIGINT,
    updated_by BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, key)
);

CREATE TABLE IF NOT EXISTS prm_template_version (
    id BIGSERIAL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    template_id BIGINT NOT NULL REFERENCES prm_template(id) ON DELETE CASCADE,
    version_label VARCHAR(64) NOT NULL,
    content TEXT NOT NULL,
    model_hint VARCHAR(128),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    published_at TIMESTAMPTZ,
    created_by BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (template_id, version_label)
);

CREATE TABLE IF NOT EXISTS prm_template_variable (
    id BIGSERIAL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    template_version_id BIGINT NOT NULL REFERENCES prm_template_version(id) ON DELETE CASCADE,
    name VARCHAR(128) NOT NULL,
    var_type VARCHAR(32) NOT NULL DEFAULT 'string',
    required BOOLEAN NOT NULL DEFAULT false,
    default_value TEXT,
    description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    UNIQUE (template_version_id, name)
);

CREATE TABLE IF NOT EXISTS prm_usage_event (
    id BIGSERIAL PRIMARY KEY,
    tenant_id BIGINT NOT NULL,
    template_id BIGINT REFERENCES prm_template(id),
    template_version_id BIGINT REFERENCES prm_template_version(id),
    actor_id BIGINT,
    surface VARCHAR(64) NOT NULL,
    context JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_prm_template_tenant_status ON prm_template (tenant_id, status, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_prm_template_version_template ON prm_template_version (template_id, status, created_at DESC);
