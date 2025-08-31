import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm()],
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8090",
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
});
