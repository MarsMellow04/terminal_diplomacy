use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // Create a buffer 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer ).expect("Failed to read the buffer message");
    let request = String::from_utf8_lossy(&buffer);
    println!("Recieved message {}", request);
    let response = "Hello Client!".as_bytes();
    stream.write(response).expect("Failed to write the response");
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
    .expect("Failed to bind address");
    println!("Server listening on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
