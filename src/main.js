import { setKeybinding } from "./keybinding.js"
import { invoke } from "@tauri-apps/api/core"
import { devUtils } from "./dev.js"
import { drpc } from "./drpc.js"
import { handleDeeplinks } from "./deeplink.js"

handleDeeplinks()
setKeybinding()
devUtils()
drpc()

document.addEventListener("DOMContentLoaded", () => {

    // External links
    document.body.addEventListener("click", async e => {
        const target = e.target.closest('a[target="_blank"]')
        if (!target) return
        e.preventDefault()
        await invoke("open_url", { url: target.href })
    })
})