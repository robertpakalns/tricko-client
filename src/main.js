import { setKeybinding } from "./keybinding.js"
import { invoke } from "@tauri-apps/api/core"
import { drpc } from "./drpc.js"
import { handleDeeplinks } from "./deeplink.js"

handleDeeplinks()
setKeybinding()
drpc()

// Logging requests
const _fetch = window.fetch
window.fetch = (...args) => {
    const [url, ...options] = args
    const newUrl = new URL(url)
    if (newUrl.searchParams.has("log")) newUrl.searchParams.set("log", "client")
    return _fetch(newUrl, ...options)
}

document.addEventListener("DOMContentLoaded", () => {

    // External links
    document.body.addEventListener("click", async e => {
        const target = e.target.closest('a[target="_blank"]')
        if (!target) return
        e.preventDefault()
        await invoke("open_url", { url: target.href })
    })
})