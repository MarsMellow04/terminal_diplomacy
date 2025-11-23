use std::error::Error;
use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;

// Data contains helper functions related to connecting or adding/deteleting for the db
mod data;
use data::connection_pool::ConnectionPool;
use data::add_user;
use data::user::Model as UserModel;

//Connection Mnagaer stuff
mod auth;
use auth::connections_manager::ConnectionsManager;

// Use this for the game stuff
pub mod game;

use crate::data::user;
use crate::game::game_repository::GameRepository;
use crate::game::game_service::{self, GameService};

async fn handle_client(mut stream: TcpStream, cm: Arc<ConnectionsManager>) -> Result<(), Box<dyn Error>> {
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

    let buf_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buf[..n]);
    let data: Vec<String> = buf_str.split(";")
                .map(|x| x.to_string().replace("\n", ""))
                .collect();

    println!("Received message: {:?}", data);
    let command = data[0].clone();

    match command.as_str() {
        "LOGIN" => {
            let username = data[1].clone();
            let password = data[2].clone();
            let query_result = cm.handle_login(username, password).await;
            match query_result {
                Ok(Some(user)) => {
                    println!("Found some user {:?}", user);
                }
                Ok(None) => {
                    println!("No user found");
                } 
                Err(e) => {
                    eprintln!("Lol databse error {}", e);
                }
            }
        }
        "REGISTER" => {
            let username = data[1].clone();
            let password = data[2].clone();
            let query_result = cm.handle_registration(username, password).await?;
        }
        "JOIN" => {
            let game_id =  data[1].clone();
            cm.handle_join(&game_id).await;
        }
        "CREATE" => {
            cm.handle_create().await;
        }
        _ => {
            eprintln!("Malformed login message");
            return Err("Malformed login message".into());
        }
    };

    let response = b"Hello Client!";
    if let Err(e) = stream.write_all(response).await {
        eprintln!("Error writing response: {:?}", e);
        return Err(Box::new(e));
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    const CONNECTION_STRING: &str = "postgresql://postgres:mysecretpassword@localhost:5433/postgres";
    let pool = Arc::new(ConnectionPool::connect(CONNECTION_STRING).await);
    let game_repo = Arc::new(GameRepository::new(pool.clone()));
    let game_service:Arc<GameService> = Arc::new(GameService::new(game_repo));
    let cm = Arc::new(ConnectionsManager::new(pool, game_service));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let cm_clone = cm.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, cm_clone).await {
                eprintln!("Client error: {e:?}");
            }
        });
    }
}
