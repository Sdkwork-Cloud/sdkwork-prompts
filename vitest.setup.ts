import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();

function loadJson(file) {
  const path = join(root, file);
  if (!existsSync(path)) return null;
  return JSON.parse(readFileSync(path, "utf8"));
}

function loadYaml(file) {
  const path = join(root, file);
  if (!existsSync(path)) return null;
  return readFileSync(path, "utf8");
}

globalThis.forumTestFixtures = {
  root,
  loadJson,
  loadYaml,
  fileExists: (file) => existsSync(join(root, file)),
  readFile: (file) => readFileSync(join(root, file), "utf8"),
  routeManifests: {
    appApi: loadJson("sdks/_route-manifests/app-api/sdkwork-routes-prompts-app-api.route-manifest.json"),
    backendApi: loadJson("sdks/_route-manifests/backend-api/sdkwork-routes-prompts-backend-api.route-manifest.json"),
    openApi: loadJson("sdks/_route-manifests/open-api/sdkwork-routes-prompts-open-api.route-manifest.json"),
  },
  assemblies: {
    appSdk: loadJson("sdks/sdkwork-prompts-app-sdk/.sdkwork-assembly.json"),
    backendSdk: loadJson("sdks/sdkwork-prompts-backend-sdk/.sdkwork-assembly.json"),
    openSdk: loadJson("sdks/sdkwork-prompts-sdk/.sdkwork-assembly.json"),
  },
  sdkgenConfigs: {
    appApi: loadYaml("sdks/sdkwork-prompts-app-sdk/openapi/sdkwork-prompts-app-api.sdkgen.yaml"),
    backendApi: loadYaml("sdks/sdkwork-prompts-backend-sdk/openapi/sdkwork-prompts-backend-api.sdkgen.yaml"),
    openApi: loadYaml("sdks/sdkwork-prompts-sdk/openapi/sdkwork-prompts-open-api.sdkgen.yaml"),
  },
  schema: loadYaml("specs/forum-database.schema.yaml"),
  apiServer: {
    routeCount: 66,
    surfaces: ["app-api", "backend-api", "open-api"],
    prefixes: {
      "app-api": "/app/v3/api",
      "backend-api": "/backend/v3/api",
      "open-api": "/prompts/v3/api",
    },
  },
};

console.log("forum test fixtures loaded");
