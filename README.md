# Tricko Client
A desktop client for Tricko.pro

### Features
* Developer Tools (`F12`)
* Discord RPC Support
* Fullscreen (`F11`)

### Dependencies
Tricko Client is based on Rust programming language (`tao` + `wry`) with `WiX` installer support (`.msi`). For more information, check `Cargo.toml` file.

### Build
```bash
cargo run
cargo build --release
wix build build.wxs -out ./dist/tricko-client-win.msi
```

by robertpakalns | [Community Server](https://discord.gg/yPjrUrvSzv) | Powered by Tricko