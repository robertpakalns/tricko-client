import { getCurrent } from "@tauri-apps/plugin-deep-link"
import { setKeybinding } from "./keybinding.js"
import { invoke } from "@tauri-apps/api/core"
import { devUtils } from "./dev.js"
import { drpc } from "./drpc.js"

setKeybinding()
devUtils()
drpc()

document.addEventListener("DOMContentLoaded", async () => {
    document.querySelectorAll("h2").forEach(el => el.style.color = "lime")

    document.body.addEventListener("click", async e => {
        const target = e.target.closest('a[target="_blank"]')
        if (!target) return
        e.preventDefault()
        await invoke("open_url", { url: target.href })
    })

    // Deeplink
    if (!sessionStorage.getItem("deeplinkProcessed")) return
    const deeplinks = await getCurrent()
    if (!deeplinks || deeplinks.length === 0) return

    const path = deeplinks[0].replace("tricko://", "")
    const newPath = "https://tricko.pro/" + path

    if (window.location.href === newPath) return

    sessionStorage.setItem("deeplinkProcessed", "true")
    window.location.href = newPath
})