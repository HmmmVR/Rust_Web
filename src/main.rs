pub mod core;
pub mod app;

fn main() {

	let router = core::router::Router::new();

    let server = core::server::WebServer::new(&router);
    server.serve();

}

