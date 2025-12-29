use crate::commands::util::Command;
use std::{io::Write, net::TcpStream};

#[derive(Default)]
pub struct ConnectCommand {
    host: String,
    port: String,
}

impl ConnectCommand {
    pub fn new(host: String, port: String) -> Self {
        Self { host, port }
    }
}

impl Command for ConnectCommand {
    fn execute(&self) -> bool{
        let formatted_address = format!("{}:{}", self.host, self.port);
        let mut stream = TcpStream::connect(formatted_address).expect("Failed to connect");
        let response = "Hello Client!".as_bytes();
        stream.write(response).expect("Failed to write the message");
        // Establish Connection 
        // Save Connection details 
        true
    }
}