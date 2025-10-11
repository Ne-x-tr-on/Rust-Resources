use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

// Shared list of clients
type Clients = Arc<Mutex<Vec<TcpStream>>>;

fn handle_client(mut stream: TcpStream, clients: Clients) {
    let mut buffer = [0; 512];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break, // Connection closed
            Ok(n) => n,
            Err(_) => break,
        };

        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received: {}", message);

        // Broadcast the message to all other clients
        let mut clients_guard = clients.lock().unwrap();
        for client in clients_guard.iter_mut() {  // <- iter_mut() here
            let _ = client.write_all(message.as_bytes());
        }
    }

    println!("Client disconnected");
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").expect("Failed to bind address");
    println!("🚀 Chat server running on 127.0.0.1:8080");

    let clients: Clients = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut clients_guard = clients.lock().unwrap();
                clients_guard.push(stream.try_clone().expect("Failed to clone client stream"));

                let clients_clone = Arc::clone(&clients);
                thread::spawn(move || handle_client(stream, clients_clone));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
