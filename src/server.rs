use rand::seq::SliceRandom;
use serde_json::{from_str, Map, Value};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::{Connection, Headers};

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn bind(addr: &str) -> Result<Server, std::io::Error> {
        let mut ip: Vec<SocketAddr> = tokio::net::lookup_host(addr).await?.collect();
        let mut rng = rand::thread_rng();
        ip.shuffle(&mut rng);
        drop(rng); // I don't know if this increases performance but who cares I am doing it anyways

        let listener;
        let mut indx: usize = 0;
        loop {
            let getaddr = match ip.get(indx) {
                Some(i) => i,
                None => return Err(std::io::Error::from(std::io::ErrorKind::ConnectionRefused)),
            };
            let list = TcpListener::bind(getaddr).await;
            match list {
                Ok(o) => {
                    listener = o;
                    break;
                }
                Err(_) => indx += 1,
            }
        }
        Ok(Server { listener })
    }
    pub async fn accept(&self) -> Result<Connection, std::io::Error> {
        loop {
            let (mut stream, addr) = self.listener.accept().await?;
            match Self::handle(&mut stream, addr).await {
                Ok(o) => {
                    return Ok(o);
                }
                Err(_) => {
                    stream.shutdown().await?;
                }
            }
        }
    }
    async fn handle(
        stream: &mut TcpStream,
        addr: SocketAddr,
    ) -> Result<Connection, std::io::Error> {
        let mut path = String::new();
        match tokio::time::timeout(Duration::from_secs(5), stream.read_to_string(&mut path)).await {
            Ok(o) => o,
            Err(_) => {
                return Err(std::io::Error::from(std::io::ErrorKind::TimedOut));
            }
        }?;
        let mut headers_str = String::new();
        match tokio::time::timeout(
            Duration::from_secs(5),
            stream.read_to_string(&mut headers_str),
        )
        .await
        {
            Ok(o) => o,
            Err(_) => {
                return Err(std::io::Error::from(std::io::ErrorKind::TimedOut));
            }
        }?;
        let v: Map<String, Value> = from_str::<Value>(&headers_str)?
            .as_object()
            .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData))?
            .clone();
        let mut hv = Headers::new();
        for (k, v) in v {
            let tostr = v
                .as_str()
                .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData));
            let s = tostr?;
            hv.set(k, s.into())
        }
        return Ok(Connection {
            headers: hv,
            address: addr,
        });
    }
}
