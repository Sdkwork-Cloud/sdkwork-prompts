# Server Specs

Component spec for `sdkwork-prompts-api-server`.

- **Crate type**: HTTP API server process
- **Domain**: intelligence
- **Capability**: forum
- **Surfaces**: app-api (22 routes), backend-api (36 routes), open-api (8 routes)
- **Total routes**: 66
- **Public exports**: compose_prm_api_routes(), PromptsRouteInfo, route_count(), find_route()
- **Dependencies**: sdkwork-router-prompts-app-api, sdkwork-router-prompts-backend-api, sdkwork-router-prompts-open-api
