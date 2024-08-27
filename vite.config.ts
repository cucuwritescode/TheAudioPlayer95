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
        '/Users/cucu/Documents/GitHub/TheAudioPlayer', // Add your project path here
        '/Users/cucu/node_modules/@react95/core', // Allow Vite to serve from here as well
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