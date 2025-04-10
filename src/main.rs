#![windows_subsystem = "windows"]

use std::sync::{Arc, Mutex};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use wry::WebViewBuilder;

mod utils;

fn main() {
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

    let window_clone = window.clone();
    let builder = WebViewBuilder::new()
        .with_url("http://tricko.pro")
        .with_ipc_handler(move |message| {
            if message.body() == "toggle_fullscreen" {
                let window = window_clone.lock().unwrap();
                if window.fullscreen().is_some() {
                    window.set_fullscreen(None);
                } else {
                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                }
            }
        });

    let js_code: &str = include_str!("../frontend/script.js");
    let webview = builder.build(&*window.lock().unwrap()).unwrap();
    webview.evaluate_script(&js_code).unwrap();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });
}
