// DEV
const isDev = window.location.href.startsWith("http://localhost:5173") // Strict port

export const devUtils = () => {
    console.log({ isDev, href: window.location.href })
    if (!isDev) return
    console.log("setting dev environment...")

    const _fetch = window.fetch
    window.fetch = function (...args) {
        const [url, ...a] = args
        if (url.startsWith("https://api.tricko.pro")) return _fetch(url.replace("https://api.tricko.pro", "/api"), ...a)
        return _fetch(...args)
    }
}