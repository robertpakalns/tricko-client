use tauri::{
    generate_context, generate_handler, Builder, WebviewUrl, WebviewWindowBuilder, Window,
};

mod discord;

#[tauri::command]
fn toggle_fullscreen(window: Window) {
    let is_fullscreen = window.is_fullscreen().unwrap_or(false);
    window.set_fullscreen(!is_fullscreen).unwrap();
}

pub fn run() {
    Builder::default()
        .invoke_handler(generate_handler![
            toggle_fullscreen,
            discord::drpc_init,
            discord::drpc_set_detail
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                WebviewWindowBuilder::new(
                    app,
                    "main",
                    WebviewUrl::External("http://localhost:5173".parse().unwrap()),
                )
                .title("Tricko Client")
                .build()
                .unwrap();
            }

            #[cfg(not(debug_assertions))]
            {
                let script = include_str!("../../frontend-dist/script.js");

                WebviewWindowBuilder::new(
                    app,
                    "main",
                    WebviewUrl::External("https://tricko.pro".parse().unwrap()),
                )
                .title("Tricko Client")
                .initialization_script(script)
                .build()
                .unwrap();
            }

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
