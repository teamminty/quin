use crate::{Connection, Headers};
use rand::seq::SliceRandom;
use serde_json::{from_str, Map, Value};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

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
            let (stream, addr) = self.listener.accept().await?;
            match Self::handle(stream, addr).await {
                Ok(o) => {
                    return Ok(o);
                }
                Err((_, mut s)) => {
                    s.shutdown().await?;
                }
            }
        }
    }
    async fn handle(
        mut stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<Connection, (std::io::Error, TcpStream)> {
        let mut path = String::new();
        match match tokio::time::timeout(Duration::from_secs(5), stream.read_to_string(&mut path))
            .await
        {
            Ok(o) => o,
            Err(_) => {
                return Err((std::io::Error::from(std::io::ErrorKind::TimedOut), stream));
            }
        } {
            Ok(_) => {}
            Err(e) => return Err((e, stream)),
        };
        let mut headers_str = String::new();
        match match tokio::time::timeout(
            Duration::from_secs(5),
            stream.read_to_string(&mut headers_str),
        )
        .await
        {
            Ok(o) => o,
            Err(_) => {
                return Err((std::io::Error::from(std::io::ErrorKind::TimedOut), stream));
            }
        } {
            Ok(_) => {}
            Err(e) => return Err((e, stream)),
        }
        let v: Map<String, Value> = match match from_str::<Value>(&headers_str) {
            Ok(o) => o,
            Err(e) => return Err((e.into(), stream)),
        }
        .as_object()
        {
            Some(o) => o,
            None => {
                return Err((
                    std::io::Error::from(std::io::ErrorKind::InvalidData),
                    stream,
                ));
            }
        }
        .clone();
        let mut hv = Headers::new();
        for (k, v) in v {
            let tostr = v
                .as_str()
                .ok_or(std::io::Error::from(std::io::ErrorKind::InvalidData));
            let s = match tostr {
                Ok(o) => o,
                Err(e) => return Err((e, stream)),
            };
            hv.set(k, s.into())
        }
        return Ok(Connection {
            headers: hv,
            address: addr,
            stream,
        });
    }
}
