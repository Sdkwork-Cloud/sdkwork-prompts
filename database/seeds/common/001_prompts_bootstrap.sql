-- Standard forum bootstrap seed (idempotent via fixed primary keys).
-- Loaded by sdkwork-database lifecycle seed profile `standard`.

INSERT INTO prm_space (
    id, uuid, tenant_id, organization_id, data_scope, status, version,
    created_at, updated_at, deleted_at, deleted_by,
    code, slug, name, description, visibility, default_locale, settings
) VALUES (
    1,
    '00000000-0000-0000-0000-000000000001',
    100001, 0, 'default', 'active', 1,
    '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', NULL, NULL,
    'default', 'default', 'Default Prompts', NULL, 'public', NULL, '{}'::jsonb
) ON CONFLICT (id) DO NOTHING;

INSERT INTO prm_node (
    id, uuid, tenant_id, organization_id, data_scope, status, version,
    created_at, updated_at, deleted_at, deleted_by,
    space_id, parent_id, node_type, slug, name, description,
    path, level_no, sort_order, icon_media_id, cover_media_id, settings
) VALUES (
    10,
    '00000000-0000-0000-0000-000000000010',
    100001, 0, 'default', 'active', 1,
    '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', NULL, NULL,
    1, 0, 'category', 'general', 'General', NULL,
    '/10', 0, 0, NULL, NULL, '{}'::jsonb
) ON CONFLICT (id) DO NOTHING;

INSERT INTO prm_node (
    id, uuid, tenant_id, organization_id, data_scope, status, version,
    created_at, updated_at, deleted_at, deleted_by,
    space_id, parent_id, node_type, slug, name, description,
    path, level_no, sort_order, icon_media_id, cover_media_id, settings
) VALUES (
    20,
    '00000000-0000-0000-0000-000000000020',
    100001, 0, 'default', 'active', 1,
    '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', NULL, NULL,
    1, 10, 'board', 'discussion', 'Discussion', NULL,
    '/10/20', 1, 0, NULL, NULL, '{}'::jsonb
) ON CONFLICT (id) DO NOTHING;

INSERT INTO prm_node (
    id, uuid, tenant_id, organization_id, data_scope, status, version,
    created_at, updated_at, deleted_at, deleted_by,
    space_id, parent_id, node_type, slug, name, description,
    path, level_no, sort_order, icon_media_id, cover_media_id, settings
) VALUES (
    21,
    '00000000-0000-0000-0000-000000000021',
    100001, 0, 'default', 'active', 1,
    '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', NULL, NULL,
    1, 10, 'board', 'q-and-a', 'Q&A', NULL,
    '/10/21', 1, 1, NULL, NULL, '{}'::jsonb
) ON CONFLICT (id) DO NOTHING;

INSERT INTO prm_board_profile (
    id, uuid, tenant_id, organization_id, data_scope, status, version,
    created_at, updated_at, deleted_at, deleted_by,
    node_id, topic_create_mode, reply_create_mode, default_topic_sort,
    moderation_mode, attachment_policy, board_rules
) VALUES (
    30,
    '00000000-0000-0000-0000-000000000030',
    100001, 0, 'default', 'active', 1,
    '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', NULL, NULL,
    20, 'open', 'open', 'latest',
    'post', '{}'::jsonb, '{}'::jsonb
) ON CONFLICT (id) DO NOTHING;

INSERT INTO prm_board_profile (
    id, uuid, tenant_id, organization_id, data_scope, status, version,
    created_at, updated_at, deleted_at, deleted_by,
    node_id, topic_create_mode, reply_create_mode, default_topic_sort,
    moderation_mode, attachment_policy, board_rules
) VALUES (
    31,
    '00000000-0000-0000-0000-000000000031',
    100001, 0, 'default', 'active', 1,
    '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z', NULL, NULL,
    21, 'open', 'open', 'latest',
    'post', '{}'::jsonb, '{}'::jsonb
) ON CONFLICT (id) DO NOTHING;
