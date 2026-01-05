use async_trait::async_trait;
use mockall::automock;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
pub enum CommandError {
    ConnectionFailure,
    NoSessionToken,
    WriteFailure,
    NoSessionTokenRead,
    SessionSaveFailed,
    NoContextFound,
}

#[automock]
#[async_trait]
pub trait Client: Send {
    async fn send(&mut self, msg: &str) -> Result<(), CommandError>;
    async fn read(&mut self) -> Result<String, CommandError>;
}

#[async_trait]
pub trait Command {
    async fn execute(&mut self) -> Result<(), CommandError>;
}

pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    pub async fn connect(addr: &str) -> Result<Self, CommandError> {
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|_| CommandError::ConnectionFailure)?;
        Ok(Self { stream })
    }
}

#[async_trait]
impl Client for TcpClient {
    async fn send(&mut self, msg: &str) -> Result<(), CommandError> {
        self.stream
            .write_all(msg.as_bytes())
            .await
            .map_err(|_| CommandError::WriteFailure)
    }

    async fn read(&mut self) -> Result<String, CommandError> {
        let mut buf = vec![0u8; 1024];
        let n: usize = self.stream
            .read(&mut buf)
            .await
            .map_err(|_| CommandError::NoSessionTokenRead)?;

        Ok(String::from_utf8_lossy(&buf[..n]).trim().to_string())
    }
}

#[async_trait::async_trait]
impl<T> Client for &mut T
where
    T: Client + Send + ?Sized,
{
    async fn send(&mut self, msg: &str) -> Result<(), CommandError> {
        (**self).send(msg).await
    }

    async fn read(&mut self) -> Result<String, CommandError> {
        (**self).read().await
    }
}

