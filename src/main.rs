#![windows_subsystem = "windows"]

mod discord;
mod server;
mod utils;

use std::{
    env,
    sync::{mpsc, Arc, Mutex},
};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use url::{form_urlencoded, Url};
use wry::WebViewBuilder;

enum Command {
    ToggleFullscreen,
    ToggleDevtools,
}

fn main() {
    let (webview_tx, webview_rx) = mpsc::channel::<Command>();
    let (discord_tx, discord_rx) = mpsc::channel::<String>();

    discord::discord_rpc(discord_rx);

    let window_width: i32 = 800;
    let window_height: i32 = 600;

    let event_loop = EventLoop::new();
    let window = Arc::new(Mutex::new(
        WindowBuilder::new()
            .with_title("Tricko.pro")
            .with_position(utils::center_window(window_width, window_height))
            .with_inner_size(LogicalSize::new(window_width, window_height))
            .with_window_icon(Some(utils::load_icon()))
            .build(&event_loop)
            .unwrap(),
    ));

    let webview = Arc::new(Mutex::new(None));

    *webview.lock().unwrap() = Some(
        WebViewBuilder::new()
            .with_url("http://tricko.pro")
            .build(&*window.lock().unwrap())
            .unwrap(),
    );

    webview
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .evaluate_script(include_str!("../frontend/script.js"))
        .unwrap();

    server::start_server(webview_tx, discord_tx);

    let window_clone = Arc::clone(&window);
    let webview_clone = Arc::clone(&webview);

    // Protocol
    if let Some(protocol_url) = env::args().nth(1) {
        if let Ok(url) = Url::parse(&protocol_url) {
            if url.scheme() == "tricko" {
                if let Some(value) = form_urlencoded::parse(url.query().unwrap_or("").as_bytes())
                    .find(|(key, _)| key == "url")
                    .map(|(_, v)| v.into_owned())
                {
                    let new_url = format!("https://tricko.pro{}", value);
                    if let Some(webview) = &*webview.lock().unwrap() {
                        webview
                            .evaluate_script(&format!(
                                "{}, window.location.href = '{}';",
                                include_str!("../frontend/script.js"),
                                new_url
                            ))
                            .unwrap();
                    }
                }
            }
        }
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        while let Ok(command) = webview_rx.try_recv() {
            match command {
                Command::ToggleFullscreen => {
                    let window = window_clone.lock().unwrap();
                    let is_fullscreen: bool = window.fullscreen().is_some();
                    if is_fullscreen {
                        window.set_fullscreen(None);
                    } else {
                        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                    }
                }
                Command::ToggleDevtools => {
                    if let Some(webview) = &*webview_clone.lock().unwrap() {
                        webview.open_devtools();
                    }
                }
            }
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
