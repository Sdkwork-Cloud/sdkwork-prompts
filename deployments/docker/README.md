# Docker Deployment

Docker configurations for forum API server and worker.

## Files

- `Dockerfile.server` - API server container
- `Dockerfile.worker` - Worker container
- `docker-compose.yml` - Full stack with PostgreSQL and OpenSearch

## Run

```bash
cd deployments/docker
docker-compose up -d
```

## Services

- `forum-api` - API server on port 8080
- `forum-worker` - Background worker
- `postgres` - PostgreSQL 16 on port 5432
- `search` - OpenSearch 2 on port 9200
