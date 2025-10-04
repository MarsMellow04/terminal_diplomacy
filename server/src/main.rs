use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub mod data;
use data::start_database;
use data::add_user;
use sea_orm::DatabaseConnection;

async fn handle_client(mut stream: TcpStream, connection: &DatabaseConnection) {
    // Create a buffer 
    let mut buffer = [0; 1024];
    stream.read(&mut buffer ).expect("Failed to read the buffer message");
    let request = String::from_utf8_lossy(&buffer);
    let message = request.to_string();
    println!("Recieved message {}", request);
    if message.contains("Login,") {
        let parts: Vec<&str> = message.strip_prefix("Login, ").expect("Could not parse message").split(':').collect();
        
        add_user(parts[0].to_string(), parts[1].to_string(), connection).await.unwrap()
    };
    let response = "Hello Client!".as_bytes();
    stream.write(response).expect("Failed to write the response");
}

#[tokio::main]
async fn main() {
    let connection: Arc<DatabaseConnection> = Arc::new(start_database().await.expect("Failed to start to database"));

    let listener = TcpListener::bind("127.0.0.1:8080")
    .expect("Failed to bind address");
    println!("Server listening on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connection = Arc::clone(&connection);
                tokio::spawn(async move {
                    handle_client(stream, &connection).await;
                });
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
