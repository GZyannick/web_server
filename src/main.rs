use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

mod http;
use http::request::Request;
use http::response::Response;

async fn process_socket(socket: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await.unwrap();
    let req = Request::new(&mut buffer).await.unwrap();
    let res = Response::new().await;

    socket
        .write_all(format!("{}", res).as_bytes())
        .await
        .unwrap();
    socket.flush().await.unwrap();
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        process_socket(&mut socket).await;
    }
}
