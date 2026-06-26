import { readFileSync } from "node:fs";
import { join } from "node:path";

const root = process.cwd();
const schema = readFileSync(join(root, "database/contract/schema.yaml"), "utf8");
const aiSchema = readFileSync(join(root, "specs/prompts-ai-database.schema.yaml"), "utf8");
const ddl = readFileSync(
  join(root, "database/ddl/baseline/postgres/0001_prompts_ai_baseline.sql"),
  "utf8"
);

const required = [
  "ai_prompt_category",
  "ai_prompt",
  "ai_prompt_version",
  "ai_prompt_binding",
  "ai_prompt_template",
  "ai_prompt_usage",
];

for (const table of required) {
  if (!schema.includes(`name: ${table}`)) {
    console.error(`contract schema missing table: ${table}`);
    process.exit(1);
  }
  if (!aiSchema.includes(`name: ${table}`)) {
    console.error(`prompts-ai-database.schema.yaml missing table: ${table}`);
    process.exit(1);
  }
  if (!ddl.includes(`CREATE TABLE IF NOT EXISTS ${table}`)) {
    console.error(`baseline DDL missing table: ${table}`);
    process.exit(1);
  }
}

console.log("prompts ai schema tests passed");
