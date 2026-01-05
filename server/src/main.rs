use std::error::Error;
use std::io::{Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;
use std::sync::{Arc};
use tokio::sync::RwLock;

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

//Use this for the order stuff
pub mod order;

use crate::auth::session::InMemoryStore;
use crate::data::user;
use crate::game::game_repository::GameRepository;
use crate::game::game_service::{self, GameService};
use crate::order::order_repository::OrderRepository;
use crate::order::order_service::{self, OrderService};

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
            // Need to figure out how to print this back
            let uuid_str = query_result.to_string();

            println!("[DEBUG] This is what sesssion id should look like: {uuid_str}");
            stream.write_all(uuid_str.as_bytes()).await?;
            stream.write_all(b"\n").await?;

        }
        "JOIN" => {
            println!("[DEBUG] Recieved: {:?}", data);
            let session_str = data[1].clone();
            let game_id =  data[2].clone();
            let session_id = Uuid::parse_str(&session_str)?;
            let result_id = cm.handle_join(&game_id, session_id).await?;

            println!("[DEBUG] This is what the sesssion id should look like: {result_id}");
            stream.write_all(format!("{result_id}\n").as_bytes()).await?;
            stream.write_all(b"\n").await?;
        }
        "CREATE" => {
            println!("[DEBUG] Recieved: {:?}", data);
            let session_str = data[1].clone();
            let session_id = Uuid::parse_str(&session_str)?;
            let result_id = cm.handle_create(session_id).await?;

            println!("[DEBUG] This is what the sesssion id should look like: {result_id}");
            stream.write_all(format!("{result_id}\n").as_bytes()).await?;
            stream.write_all(b"\n").await?;
        }
        "ORDER" => {
        // ORDER;MAIN;<session_id>;<orders>\n
            println!("[DEBUG] Recieved: {:?}", data);
            let phase = data[1].clone();
            let session_str = data[2].clone();
            let orders = data[3].clone();

            match phase.as_str() {
                "MAIN" => {
                    let session_id = Uuid::parse_str(&session_str)?;
                    let result_id = cm.handle_order(session_id, &orders).await?;
                    return Ok(());
                }
                _ => {
                    return Err("Malformed login message".into());
                }
            }
        }
        _ => {
    
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    const CONNECTION_STRING: &str = "postgresql://postgres:mysecretpassword@localhost:5433/postgres";
    let pool = Arc::new(ConnectionPool::connect(CONNECTION_STRING).await);
    let game_repo = Arc::new(GameRepository::new(pool.clone()));
    let order_repo = Arc::new(OrderRepository::new(pool.clone()));
    let game_service:Arc<GameService> = Arc::new(GameService::new(game_repo));
    let order_service: Arc<OrderService> = Arc::new(OrderService::new(order_repo));
    let session_store = Arc::new(RwLock::new(InMemoryStore::new()));
    let cm = Arc::new(ConnectionsManager::new(pool, session_store, game_service, order_service));

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
