// DEV
const _fetch = window.fetch
window.fetch = function (...args) {
    const [url, ...a] = args
    if (url.startsWith("https://api.tricko.pro")) return _fetch(url.replace("https://api.tricko.pro", "/api"), ...a)
    return _fetch(...args)
}

import { invoke } from '@tauri-apps/api/core'
console.log("injected!!!!")
document.querySelectorAll("h2").forEach(el => el.style.color = "lime")
document.addEventListener("keyup", async e => {
    if (e.key === "F11") {
        e.preventDefault()
        await invoke("toggle_fullscreen")
    }
})