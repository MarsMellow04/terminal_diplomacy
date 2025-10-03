use crate::Command;
use std::{io::Write, net::TcpStream};

use common::hash::hash_password;

#[derive(Default)]
pub struct LoginCommand {
    username: String,
    password: String,
}

impl LoginCommand {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

impl Command for LoginCommand {
    fn execute(&self) -> bool{
        let host = String::from("127.0.0.1");
        let port = String::from("8080");
        let formatted_address = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(formatted_address).expect("Failed to connect");
        let formatted_message = format!("Login, {}:{}", self.username,hash_password(&self.password));
        let login_message = formatted_message.as_bytes();
        stream.write(login_message).expect("Failed to write the message");
        true
    }
}