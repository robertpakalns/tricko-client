use crate::Command;
use std::{sync::mpsc::Sender, thread};
use tiny_http::{Header, Method, Response, Server, StatusCode};
use urldecode::decode;
use webbrowser;

pub fn start_server(webview_tx: Sender<Command>, discord_tx: Sender<String>) {
    thread::spawn(move || {
        let server: Server = Server::http("127.0.0.1:3030").expect("Failed to start HTTP server");

        for request in server.incoming_requests() {
            let url: String = request.url().to_string();

            if request.method() == &Method::Get {
                if url == "/toggle_fullscreen" {
                    webview_tx.send(Command::ToggleFullscreen).ok();
                } else if url == "/toggle_devtools" {
                    webview_tx.send(Command::ToggleDevtools).ok();
                } else if url.starts_with("/change_path") {
                    discord_tx.send(decode(url)).ok();
                } else if url.starts_with("/open_url") {
                    if let Some(external_url) = decode(url).splitn(2, "=").nth(1) {
                        webbrowser::open(external_url).ok();
                    }
                }
            }

            let response = Response::empty(StatusCode(200))
                .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap());

            let _ = request.respond(response);
        }
    });
}
