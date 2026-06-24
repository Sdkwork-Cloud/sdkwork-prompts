import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const failures = [];

function fail(message) {
  failures.push(message);
}

function readJson(file) {
  return JSON.parse(readFileSync(join(root, file), "utf8"));
}

function read(file) {
  return readFileSync(join(root, file), "utf8");
}

console.log("Running prompts SDK tests...");

const sdkgenConfigs = [
  ["sdks/sdkwork-prompts-app-sdk/openapi/sdkwork-prompts-app-api.sdkgen.yaml", "sdkwork-prompts-app-api", "sdkwork-prompts-app-sdk", "/app/v3/api", "app-api"],
  ["sdks/sdkwork-prompts-backend-sdk/openapi/sdkwork-prompts-backend-api.sdkgen.yaml", "sdkwork-prompts-backend-api", "sdkwork-prompts-backend-sdk", "/backend/v3/api", "backend-api"],
  ["sdks/sdkwork-prompts-sdk/openapi/sdkwork-prompts-open-api.sdkgen.yaml", "sdkwork-prompts-open-api", "sdkwork-prompts-sdk", "/prompts/v3/api", "open-api"],
];

for (const [file, authority, family, prefix, surface] of sdkgenConfigs) {
  if (!existsSync(join(root, file))) {
    fail(`missing sdkgen config: ${file}`);
    continue;
  }
  const text = read(file);
  if (!text.includes(`sdkFamily: ${family}`)) fail(`${file} sdkFamily mismatch`);
  if (!text.includes(`apiAuthority: ${authority}`)) fail(`${file} apiAuthority must be ${authority}`);
  if (!text.includes("capability: prompts")) fail(`${file} capability must be prompts`);
  if (!text.includes(`prefix: ${prefix}`)) fail(`${file} prefix mismatch`);
  if (!text.includes(`surface: ${surface}`)) fail(`${file} surface mismatch`);
  if (!text.includes("standardProfile: sdkwork-v3")) fail(`${file} missing standardProfile`);
  if (!text.includes("schemaVersion: 1")) fail(`${file} missing schemaVersion`);
}

const manifestFiles = [
  "sdks/_route-manifests/app-api/sdkwork-router-prompts-app-api.route-manifest.json",
  "sdks/_route-manifests/backend-api/sdkwork-router-prompts-backend-api.route-manifest.json",
  "sdks/_route-manifests/open-api/sdkwork-router-prompts-open-api.route-manifest.json",
];

for (const file of manifestFiles) {
  if (!existsSync(join(root, file))) continue;
  const json = readJson(file);
  if (!Array.isArray(json.routes)) fail(`${file} routes must be array`);
  for (const route of json.routes) {
    if (!route.operationId) fail(`${file} route missing operationId`);
    if (!route.method) fail(`${file} route missing method`);
    if (!route.path) fail(`${file} route missing path`);
    if (!route.auth?.mode) fail(`${file} route missing auth.mode`);
  }
}

const assemblyFiles = [
  "sdks/sdkwork-prompts-app-sdk/.sdkwork-assembly.json",
  "sdks/sdkwork-prompts-backend-sdk/.sdkwork-assembly.json",
  "sdks/sdkwork-prompts-sdk/.sdkwork-assembly.json",
];

for (const file of assemblyFiles) {
  if (!existsSync(join(root, file))) continue;
  const json = readJson(file);
  if (!json.sdkOwner) fail(`${file} missing sdkOwner`);
  if (!json.apiAuthority) fail(`${file} missing apiAuthority`);
  if (!json.sdkFamily) fail(`${file} missing sdkFamily`);
  if (!json.discoverySurface) fail(`${file} missing discoverySurface`);
  if (!Array.isArray(json.sdkDependencies)) fail(`${file} sdkDependencies must be array`);
  if (file.includes("backend") && json.languages?.[0]?.name !== "@sdkwork/prompts-backend-sdk") {
    fail(`${file} typescript package name must be @sdkwork/prompts-backend-sdk`);
  }
}

const composedFacades = [
  "sdks/sdkwork-prompts-app-sdk/composed/src/index.ts",
  "sdks/sdkwork-prompts-backend-sdk/composed/src/index.ts",
  "sdks/sdkwork-prompts-sdk/composed/src/index.ts",
];

for (const file of composedFacades) {
  if (!existsSync(join(root, file))) {
    fail(`missing composed facade: ${file}`);
    continue;
  }
  const text = read(file);
  if (text.includes("throw new Error(\"TODO")) fail(`${file} contains TODO stub`);
  if (!text.includes("export class")) fail(`${file} missing exported class`);
  if (!text.includes("constructor")) fail(`${file} missing constructor`);
  if (text.includes("forum:")) fail(`${file} must not use legacy forum dependency key`);
  if (file.includes("backend") && !text.includes("definitions")) {
    fail(`${file} backend facade must expose prompt definitions API`);
  }
  if (file.includes("app-sdk") && !text.includes("templates")) {
    fail(`${file} app facade must expose prompt templates API`);
  }
  if (file === "sdks/sdkwork-prompts-sdk/composed/src/index.ts" && !text.includes("catalog")) {
    fail(`${file} open facade must expose prompt catalog API`);
  }
}

const generatedSdkTypesDir = join(
  root,
  "sdks/sdkwork-prompts-backend-sdk/generated/server-openapi/src/types",
);
if (existsSync(generatedSdkTypesDir)) {
  for (const entry of readdirSync(generatedSdkTypesDir)) {
    if (!entry.endsWith(".ts")) continue;
    const text = read(join("sdks/sdkwork-prompts-backend-sdk/generated/server-openapi/src/types", entry));
    if (/claw\s*router/i.test(text)) {
      fail(`generated backend SDK type ${entry} must not reference Claw Router`);
    }
  }
}

if (failures.length > 0) {
  console.error("SDK test failures:");
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}

console.log(`prompts SDK tests passed (${sdkgenConfigs.length} sdkgen configs, ${manifestFiles.length} manifests, ${composedFacades.length} facades verified)`);
