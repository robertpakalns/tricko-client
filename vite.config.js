import { defineConfig } from "vite"

const domain = "https://tricko.pro"
const injectedScript = '<script type="module" src="/src/main.js"></script>'

export default defineConfig({
    server: {
        proxy: {
            "/assets/": domain,
            "/": {
                target: domain,
                changeOrigin: true
            },
            "^/(?!@vite|src|node_modules|\.vite).*": {
                target: domain,
                changeOrigin: true,
                selfHandleResponse: true,
                bypass: (req, res) => {
                    if (!req.headers.accept?.includes("text/html")) return req.url
                    return fetch(domain)
                        .then(r => r.text())
                        .then(html => {
                            res.setHeader("content-type", "text/html")
                            res.end(html.replace("</body>", `${injectedScript}</body>`))
                            return true
                        })
                }
            }
        }
    },
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