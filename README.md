# Tricko Client
A desktop client for Tricko.pro

### Features
* Developer Tools (`F12`)
* Discord RPC Support
* Fullscreen (`F11`)
* Links in External Browser

### Protocol
Tricko Client uses `tricko://` protocol to open the client.  
Syntax: `tricko://?url=/path/to/page`, `tricko://?url=path/to/page`, or just `tricko://`

### Dependencies
Tricko Client is based on `Tauri` (`Rust`) framework and `Vite` (`Node.js`) frontend bundler. For more information, check `src-tauri/Cargo.toml` and `package.json`.

### Build
```bash
# Make sure Node and Rustup are installed 
npm install
npm run tauri dev
npm run tauri build
```

by robertpakalns | [Community Server](https://discord.gg/yPjrUrvSzv) | Powered by Tricko