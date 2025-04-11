const { postMessage } = window.ipc

document.addEventListener("keydown", e => {

    switch (e.key) {
        case "F11":
            e.preventDefault()
            postMessage("toggle_fullscreen")
            break
        case "F12":
            e.preventDefault()
            postMessage("toggle_devtools")
            break
        default:
            break
    }

})