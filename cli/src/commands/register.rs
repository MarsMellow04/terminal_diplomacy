use crate::Command;
use std::{io::Write, net::TcpStream};

#[derive(Default)]
pub struct RegisterCommand {
    username: String,
    password: String,
}

impl RegisterCommand {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Command for RegisterCommand {
    fn execute(&self) -> bool{
        let host = String::from("127.0.0.1");
        let port = String::from("8080");
        let formatted_address = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(formatted_address).expect("Failed to connect");
        // REGISTER;USERNAME;PASSWORD_HASH\n
        let formatted_message = format!("REGISTER;{};{}", self.username,self.password);
        let Register_message = formatted_message.as_bytes();
        stream.write(Register_message).expect("Failed to write the message");
        true
    }
}