use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

mod http_parser;
use http_parser::request::Request;

mod http_response;
use http_response::response::Response;

async fn process_socket(socket: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await.unwrap();
    //let req = Request::new(&mut buffer).await.unwrap();
    //println!("req: {:#?}", req);

    let res = Response::new().await;
    println!("{res}");

    //let _ = socket.write_all(b"Hello World");

    let _ = socket.write_all(format!("{}", res).as_bytes());
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        process_socket(&mut socket).await;
    }
}
