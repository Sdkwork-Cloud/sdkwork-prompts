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
      },
    },
    server: {
      port: 5175,
    },
  };
});
