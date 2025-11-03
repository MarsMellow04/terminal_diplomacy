use std::error::Error;
use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Data contains helper functions related to connecting or adding/deteleting for the db
mod data;
use data::connection_pool::ConnectionPool;
use data::add_user;

async fn handle_client(mut stream: TcpStream, connection: ConnectionPool) -> Result<(), Box<dyn Error>> {
    // Create a buffer
    let mut buf = [0; 1024];

    let n = match stream.read(&mut buf).await {
        Ok(0) => {
            println!("Client has disconnected");
            return Ok(()); 
        }
        Ok(n) => n, // bytes read
        Err(e) => {
            eprintln!("Error reading from socket: {:?}", e);
            return Err(Box::new(e));
        }
    };

    let request = String::from_utf8_lossy(&buf[..n]);
    let message = request.to_string();

    println!("Received message: {}", message);

    // Simple message parsing logic
    if message.contains("Login, ") {
        if let Some(rest) = message.strip_prefix("Login, ") {
            let parts: Vec<&str> = rest.split(':').collect();
            if parts.len() == 2 {
                if let Err(e) = add_user(parts[0].to_string(), parts[1].to_string(), connection.get_connection()).await {
                    eprintln!("Error adding user: {:?}", e);
                    return Err(Box::new(e)); 
                }
            } else {
                eprintln!("Malformed login message");
                return Err("Malformed login message".into());
            }
        }
    }

    let response = b"Hello Client!";
    if let Err(e) = stream.write_all(response).await {
        eprintln!("Error writing response: {:?}", e);
        return Err(Box::new(e));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    const CONNECTION_STRING: &str = "postgresql://postgres:mysecretpassword@localhost/terminal_diplomacy";
    let pool = ConnectionPool::connect(CONNECTION_STRING).await;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let pool = pool.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, pool).await {
                eprintln!("Client error: {e:?}");
            }
        });
    }
}
