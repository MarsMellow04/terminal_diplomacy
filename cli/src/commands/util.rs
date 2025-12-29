use std::{io::{Read, Write}, net::TcpStream};

use mockall::automock;


#[derive(Debug)]
pub enum CommandError {
    ConectionFailure,
    NoSessionToken,
    WriteFailure,
    NoSessionTokenRead,
    SessionSaveFailed
}

pub trait Command {
    fn execute(&self) -> bool;
}

#[automock]
pub trait Client {
    // Making send a dependency injection
    fn send(&mut self, msg: &str ) -> Result<(), CommandError>;
    fn read(&mut self) -> Result<String, CommandError>;
} 

pub struct TcpClient {
    pub stream: TcpStream
}

impl TcpClient {
    pub fn connect(addr: &str)-> Result<Self, CommandError> {
        let stream = TcpStream::connect(addr)
            .or(Err(CommandError::ConectionFailure))?;
        Ok( Self { stream })
    }
}

impl Client for TcpClient {
    fn send(&mut self, msg: &str ) -> Result<(), CommandError> {
        self.stream.write_all(msg.as_bytes())
            .or(Err(CommandError::WriteFailure))?;
        Ok(())
    }

    fn read(&mut self) -> Result<String, CommandError> {
        let mut buf = String::new();
        self.stream.read_to_string(&mut buf)
            .or(Err(CommandError::NoSessionTokenRead))?;
        Ok(buf.trim().to_string())
    }
}