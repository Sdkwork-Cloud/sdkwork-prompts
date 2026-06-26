# Open API Examples

Public catalog reads — no credentials required.

```http
GET /prompts/v3/api/prompts/catalog
```

Response shape:

```json
{
  "success": true,
  "data": {
    "items": [
      { "key": "support.greeting", "name": "Support Greeting", "description": null }
    ]
  }
}
```
