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

const updateUrl = () => {
    const url = encodeURIComponent(window.location.pathname)
    fetch(`${base_url}/change_path?url=${url}`, { method: "GET" })
}

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