import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const schema = fs.readFileSync(path.join(root, "database/contract/schema.yaml"), "utf8");
const expectedTables = [
  "ai_prompt_category",
  "ai_prompt",
  "ai_prompt_version",
  "ai_prompt_binding",
  "ai_agent_prompt_template",
  "ai_prompt_usage_event",
];

for (const table of expectedTables) {
  assert.match(schema, new RegExp(`name: ${table}`), `schema must declare ${table}`);
}

const appOpenApi = fs.readFileSync(
  path.join(root, "sdks/sdkwork-prompts-app-sdk/openapi/sdkwork-prompts-app-api.openapi.yaml"),
  "utf8",
);
assert.match(appOpenApi, /prompts\.templates\.list/, "app openapi must declare prompts.templates.list");
assert.match(appOpenApi, /x-sdkwork-request-context: WebRequestContext/, "app openapi must declare WebRequestContext");

console.log("prompts contracts validation passed");
