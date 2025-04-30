import.meta.hot?.accept()


// main code
console.log("injected!!!!")
document.querySelectorAll("h2").forEach(el => el.style.color = "lime")