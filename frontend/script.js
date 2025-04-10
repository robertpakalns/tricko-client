document.addEventListener("keydown", e => {
    if (e.key === "F11") {
        e.preventDefault()
        window.ipc.postMessage("toggle_fullscreen")
    }
})