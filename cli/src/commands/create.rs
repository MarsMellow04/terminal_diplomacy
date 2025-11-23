use crate::Command;
use std::{io::Write, net::TcpStream};

#[derive(Default)]
pub struct CreateCommand {
}

impl CreateCommand {
    pub fn new() -> Self {
        Self {}
    }
}

impl Command for CreateCommand {
    fn execute(&self) -> bool{
        // attempt to join a game
        let host = String::from("127.0.0.1");
        let port = String::from("8080");
        let formatted_address = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(formatted_address).expect("Failed to connect");
        // JOIN;GAME_ID\n
        let formatted_message = String::from("CREATE;");
        let join_message = formatted_message.as_bytes();
        stream.write(join_message).expect("Failed to write the message");
        true
    }
}