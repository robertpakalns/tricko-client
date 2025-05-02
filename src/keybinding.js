import { invoke } from "@tauri-apps/api/core"

export const setKeybinding = () => {
    document.addEventListener("keyup", async e => {
        if (e.key === "F11") {
            e.preventDefault()
            await invoke("toggle_fullscreen")
        }
    })
}