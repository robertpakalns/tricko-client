import { defineConfig } from "vite"

export default defineConfig({
    clearScreen: false,
    build: {
        target: "esnext",
        outDir: "./frontend-dist",
        emptyOutDir: true,
        rollupOptions: {
            input: "./src/main.js",
            output: {
                entryFileNames: "script.js"
            }
        }
    }
})