use tauri::{generate_context, generate_handler, Builder, Manager, Window};

#[tauri::command]
fn toggle_fullscreen(window: Window) {
    let is_fullscreen = window.is_fullscreen().unwrap_or(false);
    window.set_fullscreen(!is_fullscreen).unwrap();
}

pub fn run() {
    Builder::default()
        .invoke_handler(generate_handler![toggle_fullscreen])
        .setup(|app| {
            let main_webview = app.get_webview_window("main").unwrap();

            #[cfg(debug_assertions)]
            {
                main_webview
                    .navigate("http://localhost:5173".parse().unwrap())
                    .unwrap();
            }

            #[cfg(not(debug_assertions))]
            {
                main_webview
                    .navigate("https://tricko.pro".parse().unwrap())
                    .unwrap();
                main_webview
                    .eval(include_str!("../../frontend-dist/script.js"))
                    .unwrap();
            }

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
