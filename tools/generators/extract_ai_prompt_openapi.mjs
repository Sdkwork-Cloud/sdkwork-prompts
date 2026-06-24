import { copyFileSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const clawPath = join(
  root,
  "../sdkwork-claw-router/sdks/clawrouter-backend-sdk/openapi/clawrouter-backend-sdk.openapi.json",
);
const openapiPath = join(root, "apis/backend-api/intelligence/prompts/openapi.json");
const claw = JSON.parse(readFileSync(clawPath, "utf8"));
const doc = JSON.parse(readFileSync(openapiPath, "utf8"));
const schemaNames = new Set();

function collectRefs(obj) {
  if (!obj || typeof obj !== "object") return;
  if (typeof obj.$ref === "string" && obj.$ref.startsWith("#/components/schemas/")) {
    schemaNames.add(obj.$ref.slice("#/components/schemas/".length));
  }
  for (const value of Object.values(obj)) collectRefs(value);
}

collectRefs(doc.paths);
const schemas = {};
for (const name of [...schemaNames].sort()) {
  if (claw.components.schemas[name]) {
    schemas[name] = claw.components.schemas[name];
  } else {
    console.warn(`missing schema: ${name}`);
  }
}

doc.components = { securitySchemes: claw.components.securitySchemes, schemas };
writeFileSync(openapiPath, JSON.stringify(doc, null, 2));
const sdkTarget = join(
  root,
  "sdks/sdkwork-prompts-backend-sdk/openapi/sdkwork-prompts-backend-api.openapi.json",
);
copyFileSync(openapiPath, sdkTarget);
console.log(`prompts AI admin OpenAPI updated with ${Object.keys(schemas).length} schemas`);
