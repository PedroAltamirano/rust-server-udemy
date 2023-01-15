// disable linter rule
#![allow(dead_code)]

// use http::method::Method; large way
// use http::Method;
// use http::Request;
use server::Server;

mod http;
mod server;

fn main() {
    let host = String::from("127.0.0.1:8000");
    let server = Server::new(host);
    server.run();
}

// check string borrowing & awning
