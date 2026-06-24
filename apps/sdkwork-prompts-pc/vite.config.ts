import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react";
import path from "node:path";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, ".", "");

  return {
    define: {
      "process.env.SDKWORK_ACCESS_TOKEN": JSON.stringify(
        env.SDKWORK_ACCESS_TOKEN ?? process.env.SDKWORK_ACCESS_TOKEN ?? "",
      ),
    },
    plugins: [react()],
    root: ".",
    resolve: {
      alias: {
        "@sdkwork/prompts-pc-workspace": path.resolve(
          __dirname,
          "packages/sdkwork-prompts-pc-workspace/src",
        ),
        "@sdkwork/prompts-pc-commons/runtime": path.resolve(
          __dirname,
          "packages/sdkwork-prompts-pc-commons/src/runtime.ts",
        ),
        "@sdkwork/prompts-pc-admin-prompts": path.resolve(
          __dirname,
          "packages/sdkwork-prompts-pc-admin-prompts/src/promptService.ts",
        ),
        "@sdkwork/prompts-backend-sdk": path.resolve(
          __dirname,
          "../../sdks/sdkwork-prompts-backend-sdk/generated/server-openapi/src/index.ts",
        ),
      },
    },
    server: {
      port: 5175,
    },
  };
});
