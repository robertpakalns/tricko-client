use crate::Command;
use std::{sync::mpsc::Sender, thread};
use tiny_http::{Header, Method, Response, Server, StatusCode};

pub fn start_server(tx: Sender<Command>) {
    thread::spawn(move || {
        let server: Server = Server::http("127.0.0.1:3030").expect("Failed to start HTTP server");

        for request in server.incoming_requests() {
            let url: String = request.url().to_string();

            if request.method() == &Method::Get {
                if url == "/toggle_fullscreen" {
                    tx.send(Command::ToggleFullscreen).ok();
                } else if url == "/toggle_devtools" {
                    tx.send(Command::ToggleDevtools).ok();
                }
            }

            let response = Response::empty(StatusCode(200))
                .with_header(Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap());

            let _ = request.respond(response);
        }
    });
}
