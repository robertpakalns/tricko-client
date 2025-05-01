import { defineConfig } from "vite"

const domain = "https://tricko.pro"
const apiDomain = "https://api.tricko.pro"
const injectedScript = '<script type="module" src="/src/main.js"></script>'

export default defineConfig({
    server: {
        proxy: {
            "/assets/": domain,
            "/api/": {
                target: apiDomain,
                changeOrigin: true,
                rewrite: path => path.replace("/api", "")
            },
            "^/(?!@vite|src|node_modules|\.vite).*": {
                target: domain,
                changeOrigin: true,
                ws: true,
                selfHandleResponse: true,
                bypass: (req, res) => {
                    if (req.headers.accept?.includes("text/html")) return fetch(domain)
                        .then(r => r.text())
                        .then(html => {
                            res.setHeader("content-type", "text/html")
                            res.end(html.replace("</body>", `${injectedScript}</body>`))
                            return true
                        })

                    return req.url
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