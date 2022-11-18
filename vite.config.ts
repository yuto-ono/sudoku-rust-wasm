import { defineConfig } from "vite"
import { svelte } from "@sveltejs/vite-plugin-svelte"
import topLevelAwait from "vite-plugin-top-level-await"
import wasm from "vite-plugin-wasm"

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [wasm(), topLevelAwait(), svelte()],
})
