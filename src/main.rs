pub mod core;
pub mod app;

fn main() {

    let server = core::server::WebServer::new();
    server.serve();

}

