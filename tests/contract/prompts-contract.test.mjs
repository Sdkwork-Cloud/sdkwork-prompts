import { existsSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const required = [
  "database/contract/schema.yaml",
  "apis/app-api/intelligence/prompts/openapi.yaml",
  "apis/backend-api/intelligence/prompts/openapi.yaml",
  "apis/open-api/intelligence/prompts/openapi.yaml",
  "apis/authority-manifest.json",
];

for (const file of required) {
  if (!existsSync(join(root, file))) {
    console.error(`missing contract file: ${file}`);
    process.exit(1);
  }
}

console.log("prompts contract tests passed");
