use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };

pub struct WebServer <'a> {
    // Serve port
    port: u32,
    // Vector of headers
    headers: Vec<&'a str>
}

impl <'a> WebServer <'a> {
    // New Server
    pub fn new() -> WebServer<'a> {
        WebServer {
            port: 1337,
            headers: vec![]
        }
    }

    // Match a response code with the full response
    fn get_response_code(code: i32) -> &'a str {
        match code {
            200 => "200 OK",
            400 => "400 Bad Request",
            403 => "403 Forbidden",
            404 => "404 Not Found",
            500 => "500 Internal Server Error",
            _ => ""
        }
    }

    // Read stream
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

    // Write stream
    fn handle_write(&self, mut stream: TcpStream) {
        let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html;charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
        match stream.write(response) {
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Failed sending response: {}", e),
        }
    }

    // Read and write stream
    fn handle_client(&self, stream: TcpStream) {
        self.handle_read(&stream);
        self.handle_write(stream);
    }

    // Serve the server
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