import { existsSync, readFileSync } from "node:fs";
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

console.log("Running forum SDK tests...");

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
}

if (failures.length > 0) {
  console.error("SDK test failures:");
  for (const f of failures) console.error(`  - ${f}`);
  process.exit(1);
}

console.log(`forum SDK tests passed (${sdkgenConfigs.length} sdkgen configs, ${manifestFiles.length} manifests, ${composedFacades.length} facades verified)`);
