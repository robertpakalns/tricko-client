import { invoke } from "@tauri-apps/api/core"

const games = new Set(["cryzen", "kirka", "vectaria", "voxiom"])
const capStr = s => s ? s[0].toUpperCase() + s.slice(1) : ""
const setDetail = path => {
    const parts = path.split("=").pop().split("/").slice(1)
    const [game, section, detail] = parts

    if (!games.has(game)) return "Searching Stats"
    if (!section) return `${capStr(game)} page`
    if (!detail) return `${capStr(game)} ${section} page`

    return `${capStr(game)} ${section}: ${detail}`
}

const updateUrl = () => invoke("drpc_set_detail", { text: setDetail(window.location.pathname) })

export const drpc = async () => {
    const _pushState = history.pushState
    history.pushState = async function () {
        _pushState.apply(history, arguments)
        await updateUrl()
    }

    const _replaceState = history.replaceState
    history.replaceState = async function () {
        _replaceState.apply(history, arguments)
        await updateUrl()
    }

    window.addEventListener("popstate", updateUrl)

    await updateUrl()
}