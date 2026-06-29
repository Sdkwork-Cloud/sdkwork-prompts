# Local Config

Local development configuration templates.

## Files

- `.env.example` — environment variables for `sdkwork-prompts-standalone-gateway`

## Usage

```bash
cp configs/local/.env.example .env.local
# Edit SDKWORK_PROMPTS_DATABASE_URL and optional integration URLs
pnpm db:bootstrap
cargo run --bin sdkwork-prompts-standalone-gateway
```

## Services

- **Required**: PostgreSQL 16+ with a `prompts` database
- **Optional**: Drive (attachments), search index, notification adapter
