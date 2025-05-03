import { getCurrent } from "@tauri-apps/plugin-deep-link"

export const handleDeeplinks = async () => {
    if (sessionStorage.getItem("deeplinkProcessed")) return
    const deeplinks = await getCurrent()
    if (!deeplinks || deeplinks.length === 0) return
    const deeplinkUrl = new URL(deeplinks[0])
    const targetPath = deeplinkUrl.searchParams.get("url")

    if (!targetPath) return

    const path = "/" + targetPath.replace(/^\/|\/$/g, "")

    if (window.location.pathname === path) return

    sessionStorage.setItem("deeplinkProcessed", "true")
    window.location.pathname = path
}