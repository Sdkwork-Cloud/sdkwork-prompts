# Kubernetes Deployment

Kubernetes manifests will be added after runtime entrypoints exist.

## Status

Docker containers are defined in `deployments/docker/`. Kubernetes manifests require:
- API server binary entrypoint
- Worker binary entrypoint
- Health check endpoints
- Graceful shutdown handling

## Planned Manifests

- `deployment-api.yaml` - API server deployment
- `deployment-worker.yaml` - Worker deployment
- `service.yaml` - ClusterIP service for API
- `configmap.yaml` - Non-secret configuration
- `secret.yaml` - Secret references (external secret manager)
- `cronjob-stats.yaml` - Nightly stats rebuild
- `cronjob-search.yaml` - Hourly search repair
