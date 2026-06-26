import { spawnSync } from "node:child_process";
import { join } from "node:path";

const root = process.cwd();
const sdkgen = join(root, "../sdkwork-sdk-generator/bin/sdkgen.js");

const targets = [
  {
    input: "sdks/sdkwork-prompts-app-sdk/openapi/sdkwork-prompts-app-api.openapi.yaml",
    output: "sdks/sdkwork-prompts-app-sdk/generated/server-openapi",
    name: "sdkwork-prompts-app-sdk",
    type: "app",
    packageName: "@sdkwork/prompts-app-sdk",
    apiPrefix: "/app/v3/api",
    clientName: "SdkworkPromptsAppClient",
  },
  {
    input: "sdks/sdkwork-prompts-backend-sdk/openapi/sdkwork-prompts-backend-api.openapi.yaml",
    output: "sdks/sdkwork-prompts-backend-sdk/generated/server-openapi",
    name: "sdkwork-prompts-backend-sdk",
    type: "backend",
    packageName: "@sdkwork/prompts-backend-sdk",
    apiPrefix: "/backend/v3/api",
    clientName: "SdkworkPromptsBackendClient",
  },
  {
    input: "sdks/sdkwork-prompts-sdk/openapi/sdkwork-prompts-open-api.openapi.yaml",
    output: "sdks/sdkwork-prompts-sdk/generated/server-openapi",
    name: "sdkwork-prompts-sdk",
    type: "custom",
    packageName: "@sdkwork/prompts-sdk",
    apiPrefix: "/prompts/v3/api",
    clientName: "SdkworkPromptsOpenClient",
  },
];

for (const target of targets) {
  const args = [
    sdkgen,
    "generate",
    "-i",
    join(root, target.input),
    "-o",
    join(root, target.output),
    "-n",
    target.name,
    "-t",
    target.type,
    "-l",
    "typescript",
    "--package-name",
    target.packageName,
    "--api-prefix",
    target.apiPrefix,
    "--standard-profile",
    "sdkwork-v3",
    "--client-name",
    target.clientName,
    "--sdk-version",
    "0.1.0",
    "--no-sync-published-version",
  ];
  console.log(`sdkgen ${target.name}`);
  const result = spawnSync(process.execPath, args, { stdio: "inherit", cwd: root });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

console.log("prompts SDK generation complete");
