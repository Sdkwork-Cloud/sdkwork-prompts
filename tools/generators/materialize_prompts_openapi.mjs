import { copyFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

const root = process.cwd();

const materializations = [
  {
    source: "apis/app-api/intelligence/prompts/openapi.yaml",
    target: "sdks/sdkwork-prompts-app-sdk/openapi/sdkwork-prompts-app-api.openapi.yaml",
  },
  {
    source: "apis/backend-api/intelligence/prompts/openapi.yaml",
    target: "sdks/sdkwork-prompts-backend-sdk/openapi/sdkwork-prompts-backend-api.openapi.yaml",
  },
  {
    source: "apis/open-api/intelligence/prompts/openapi.yaml",
    target: "sdks/sdkwork-prompts-sdk/openapi/sdkwork-prompts-open-api.openapi.yaml",
  },
];

for (const item of materializations) {
  const target = join(root, item.target);
  mkdirSync(dirname(target), { recursive: true });
  copyFileSync(join(root, item.source), target);
  console.log(`${item.source} -> ${item.target}`);
}

console.log("prompts OpenAPI materialization complete");
