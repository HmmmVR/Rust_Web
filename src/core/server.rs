use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };
use core::router::Router;

pub struct Request {

}

pub struct Response {

}

pub struct WebServer <'a> {
    port: u32,
    headers: Vec<&'a str>,
	router: &<'a> Router
}

impl <'a> WebServer <'a> {
    // New Server
    pub fn new(router: &Router) -> WebServer<'a> {
        WebServer {
            port: 1337,
            headers: vec![ "Content-Type: application/json", "X-Powered-by: MEEE" ],
			router: &<'a> router
        }
    }

    // Match a response code with the full response
    fn get_response_code(&self, code: i32) -> String {
        match code {
            200 => String::from("200 OK"),
            400 => String::from("400 Bad Request"),
            403 => String::from("403 Forbidden"),
            404 => String::from("404 Not Found"),
            500 => String::from("500 Internal Server Error"),
            _ => String::from("")
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

    fn get_headers(&self) -> String {
        let mut result: String = "".to_owned();

        for header in &self.headers {
            result += &format!("{}{}", header, ";\r\n");
        }

        result
    }

    // Write stream
    fn handle_write(&self, mut stream: TcpStream) {

        let response = format!(
            "{}{}{}{}",
            "HTTP/1.1 ",
            self.get_response_code(200) + "\r\n",
            self.get_headers(),
            "\r\n{\"hello\": \"world\"}\r\n"
        );

        match stream.write(response.as_bytes()) {
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