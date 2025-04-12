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