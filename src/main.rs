use std::collections::HashMap;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

mod http;
use http::request::Request;
use http::response::Response;
use http::{error::Error, response::status_code::HttpStatusCode, version::HttpVersion};

async fn handler(socket: &mut tokio::net::TcpStream) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await?;
    let _req = Request::new(&mut buffer).await?;

    let res = Response::new(HttpVersion::Http1_1, HttpStatusCode::Ok, HashMap::new()).await?;

    socket.write_all(format!("{}", res).as_bytes()).await?;
    socket.flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        handler(&mut socket).await?;
    }
}

/*
* TODO
*
*  User defined handler
*
*  MiddleWare (Multi Thread i guess)
*
*  Radix tree routing
*
*/
