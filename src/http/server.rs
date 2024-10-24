use super::{
    error::Error, request::Request, response::status_code::HttpStatusCode, response::Response,
    version::HttpVersion,
};
use std::collections::HashMap;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(addr: &'static str) -> Result<Server, Error> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Server { listener })
    }

    pub async fn handle_connection(&self) -> Result<(), Error> {
        loop {
            let (mut socket, _) = self.listener.accept().await?;
            self.handler(&mut socket).await?;
        }
    }

    async fn handler(&self, socket: &mut tokio::net::TcpStream) -> Result<(), Error> {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await?;
        let req = Request::new(&mut buffer).await?;

        let res = Response::new(HttpVersion::Http1_1, HttpStatusCode::Ok, HashMap::new()).await?;

        socket.write_all(format!("{}", res).as_bytes()).await?;
        socket.flush().await?;

        Ok(())
    }
}
