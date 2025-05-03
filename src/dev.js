// DEV
const devURL = "http://localhost:5173"
const apiURL = "https://api.tricko.pro"
const isDev = window.location.href.startsWith(devURL) // Strict port

export const devUtils = () => {
    if (!isDev) return
    console.log("Dev mode ON")

    const _fetch = window.fetch
    window.fetch = (...args) => {
        const [url, ...options] = args
        const { href } = new URL(url)

        if (href.startsWith(apiURL)) return _fetch(href.replace(apiURL, "/api"), ...options)

        return _fetch(...args)
    }
}