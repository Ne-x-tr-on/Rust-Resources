use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from client");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received Request: {}", request);

    let response = b"Hello, Client";
    stream.write_all(response).expect("Failed to write to client");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind address");
    println!("🚀 Server running on: 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
