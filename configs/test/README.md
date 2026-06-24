# Test Config

Isolated test configuration.

## Files

- `.env.test` - Test environment variables

## Usage

```bash
cp configs/test/.env.test .env.test
```

## Notes

- Uses `prm_test` database (isolated from dev)
- Mocks external services (search, drive)
- Fast worker polling for test responsiveness
