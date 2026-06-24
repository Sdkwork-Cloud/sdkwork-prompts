# Local Config

Local development configuration templates.

## Files

- `.env.example` - Local environment variables

## Usage

```bash
cp configs/local/.env.example .env.local
# Edit .env.local with your local database/service URLs
```

## Services Required

- PostgreSQL 16+ with `forum` database
- OpenSearch 2+ (optional, for search)
- Drive service (optional, for attachments)
