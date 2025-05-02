// DEV
const devURL = "http://localhost:5173"
const apiURL = "https://api.tricko.pro"
const isDev = window.location.href.startsWith(devURL) // Strict port

export const devUtils = () => {
    if (!isDev) return
    console.log("Dev mode ON")

    const _fetch = window.fetch
    window.fetch = function (...args) {
        const [url, ...options] = args
        if (url.startsWith(apiURL)) return _fetch(url.replace(apiURL, "/api"), ...options)
        return _fetch(...args)
    }
}