# Operator guide

## Standalone deployment

### Docker Compose

From `deployments/docker/` (build context is SDKWork workspace root):

```bash
docker compose up -d --build
```

Services: `prompts-api` (8080), `postgres` (5432).

### Binary

```bash
cargo build --release --bin sdkwork-api-prompts-standalone-gateway
export SDKWORK_PROMPTS_APP_ROOT=/path/to/sdkwork-prompts
export SDKWORK_PROMPTS_DATABASE_URL=postgresql://...
./target/release/sdkwork-api-prompts-standalone-gateway
```

## Cloud topology

```bash
pnpm topology:validate
pnpm gateway:validate:cloud
```

Profiles: `specs/topology.spec.json` — standalone development and cloud split-services production.

## Production environment

See `configs/production/.env.production.example`:

- `SDKWORK_PROMPTS_IAM_ENABLED=true`
- `SDKWORK_PROMPTS_IAM_STRICT=true`
- `SDKWORK_PROMPTS_REQUIRE_AUTH=true`

## Health

Infra routes mounted via `sdkwork-web-bootstrap` (readiness on the gateway process).

## Runbooks

See [runbooks/README.md](../../runbooks/README.md).
