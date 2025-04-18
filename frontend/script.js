const base_url = "http://127.0.0.1:3030"
const sendMessage = event => fetch(`${base_url}/${event}`, { method: "GET" })

document.addEventListener("keydown", e => {
    switch (e.key) {
        case "F11":
            e.preventDefault()
            sendMessage("toggle_fullscreen")
            break
        case "F12":
            e.preventDefault()
            sendMessage("toggle_devtools")
            break
        default:
            break
    }

})

const updateUrl = () => sendMessage(`change_path?url=${encodeURIComponent(window.location.pathname)}`)

const _pushState = history.pushState
history.pushState = function () {
    _pushState.apply(history, arguments)
    updateUrl()
}

const _replaceState = history.replaceState
history.replaceState = function () {
    _replaceState.apply(history, arguments)
    updateUrl()
}

window.addEventListener("popstate", updateUrl)

updateUrl()

document.addEventListener("DOMContentLoaded", () =>
    document.body.addEventListener("click", e => {
        const target = e.target.closest('a[target="_blank"]')
        if (!target) return
        e.preventDefault()
        sendMessage(`open_url?url=${encodeURIComponent(target.href)}`)
    })
)