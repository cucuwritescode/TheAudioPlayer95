import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { resolve } from "path";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 4000,
    strictPort: true,
    fs: {
      allow: [
        '/Users/cucu/Documents/GitHub/TheAudioPlayer', 
        '/Users/cucu/node_modules/@react95/core', 
      ],
      strict: false,
    },
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    outDir: resolve(__dirname, "dist"),
    emptyOutDir: true,
    target: "esnext",
  },
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },
});