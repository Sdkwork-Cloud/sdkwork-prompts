# Production Config

Production configuration templates. DO NOT commit actual secrets.

## Files

- `.env.production.example` - Production environment variables (with secret references)

## Usage

```bash
cp configs/production/.env.production.example .env.production
# Replace <secret:...> references with actual secret manager paths
```

## Security

- Database, search, and drive URLs use secret references
- Strict token validation enabled
- Strict tenant isolation enabled
