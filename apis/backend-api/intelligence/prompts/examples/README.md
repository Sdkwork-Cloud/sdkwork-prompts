# Backend API Examples

Admin prompt governance for operators.

## Publish version

```http
POST /backend/v3/api/prompts/versions/{versionId}/publish
Authorization: Bearer <auth-token>
Access-Token: tenant_id=100001;organization_id=0;user_id=1
```

## Render version

```http
POST /backend/v3/api/prompts/versions/{versionId}/render
Authorization: Bearer <auth-token>
Access-Token: tenant_id=100001;organization_id=0;user_id=1
Content-Type: application/json

{
  "variables": {
    "product_name": "SDKWork"
  }
}
```
