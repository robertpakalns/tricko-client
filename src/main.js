// DEV
const _fetch = window.fetch
window.fetch = function (...args) {
    const [url, ...a] = args
    if (url.startsWith("https://api.tricko.pro")) return _fetch(url.replace("https://api.tricko.pro", "/api"), ...a)
    return _fetch(...args)
}

import { invoke } from "@tauri-apps/api/core"
import { setKeybinding } from "./keybinding.js"
import { drpc } from "./drpc.js"

setKeybinding()
drpc()

document.addEventListener("DOMContentLoaded", () => {
    console.log("injected!!!")
    document.querySelectorAll("h2").forEach(el => el.style.color = "lime")

    document.body.addEventListener("click", async e => {
        const target = e.target.closest('a[target="_blank"]')
        if (!target) return
        e.preventDefault()
        await invoke("open_url", { url: target.href })
    })
})