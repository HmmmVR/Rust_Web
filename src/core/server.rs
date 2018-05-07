use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::thread;

pub struct WebServer {
    port: u32
}

impl WebServer {

    pub fn new() -> WebServer {
        WebServer {
            port: 1337
        }
    }

    fn handle_read(&self, mut stream: &TcpStream) {
        let mut buf = [0u8 ;4096];
        match stream.read(&mut buf) {
            Ok(_) => {
                let req_str = String::from_utf8_lossy(&buf);
                println!("{}", req_str);
                },
            Err(e) => println!("Unable to read stream: {}", e),
        }
    }

    fn handle_write(&self, mut stream: TcpStream) {
        let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
        match stream.write(response) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
    }

    fn handle_client(&self, stream: TcpStream) {
        self.handle_read(&stream);
        self.handle_write(stream);
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port.to_string())).unwrap();
        println!("Listening for connections on port {}", self.port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    self.handle_client(stream)
                }
                Err(e) => {
                    println!("Unable to connect: {}", e);
                }
            }
        }
    }

}