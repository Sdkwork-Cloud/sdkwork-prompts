import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const schema = readFileSync(join(root, "database/contract/schema.yaml"), "utf8");
const required = [
  "prm_category",
  "prm_template",
  "prm_template_version",
  "prm_template_variable",
  "prm_usage_event",
];

for (const table of required) {
  if (!schema.includes(`name: ${table}`)) {
    console.error(`missing table: ${table}`);
    process.exit(1);
  }
}

console.log("prompts schema tests passed");
