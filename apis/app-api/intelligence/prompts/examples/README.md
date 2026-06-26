# App API Examples

Prompt template operations for authenticated app clients.

See authored OpenAPI: `apis/app-api/intelligence/prompts/openapi.yaml`

## List templates

```http
GET /app/v3/api/prompts/templates
Authorization: Bearer <auth-token>
Access-Token: tenant_id=100001;organization_id=0;user_id=1
```

## Create template version

```http
POST /app/v3/api/prompts/templates/{templateId}/versions
Authorization: Bearer <auth-token>
Access-Token: tenant_id=100001;organization_id=0;user_id=1
Content-Type: application/json

{
  "version_label": "v1",
  "content": "You are a helpful assistant for {{product_name}}."
}
```
