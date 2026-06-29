# Test Config

Isolated test configuration.

## Files

- `.env.test` — test environment variables

## Usage

```bash
cp configs/test/.env.test .env.test
pnpm test
```

## Notes

- Uses `prompts_test` database (isolated from development)
- External integrations use mock URLs
