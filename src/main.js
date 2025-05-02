// MAIN
import { invoke } from "@tauri-apps/api/core"
import { setKeybinding } from "./keybinding.js"
import { devUtils } from "./dev.js"
import { drpc } from "./drpc.js"

setKeybinding()
devUtils()
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