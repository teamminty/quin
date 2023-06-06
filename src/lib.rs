use std::{collections::HashMap, future::Future, net::SocketAddr};
use tokio::net::TcpStream;

use message::Message;

#[derive(Debug)]
pub struct Connection {
    address: SocketAddr,
    headers: Headers,
    stream: TcpStream,
}

impl Connection {
    pub fn address(&self) -> SocketAddr {
        return self.address;
    }
    pub fn headers(&self) -> &Headers {
        return &self.headers;
    }
    pub async fn send(&mut self, data: impl Into<Message>) -> Result<Message, std::io::Error> {
        let data: Message = data.into();
        todo!()
    }
    pub async fn on_message<T: Future<Output = ()>, F: Fn() -> T>(&mut self, v: F) {
        todo!()
    }
}

pub mod message;

#[derive(Default, Clone, Debug)]
pub struct Headers {
    v: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Headers {
        Self::default()
    }
    pub fn get(&self, i: &String) -> Option<&String> {
        self.v.get(i)
    }
    pub fn set(&mut self, k: String, v: String) {
        self.v.insert(k, v);
    }
}

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;
