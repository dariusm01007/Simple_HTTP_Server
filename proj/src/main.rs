// module name (files are modules in Rust) :: Struct name
// Allows for shortening the names
use server::Server;
use http::Request;
use http::Method;

// This imports the modules
mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
