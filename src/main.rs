mod http;
use http::error::Error;
use http::server::Server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let server = Server::new("127.0.0.1:8080").await?;
    server.handle_connection().await?;
    Ok(())
}

/*
* TODO
*
*  User defined handler[_]  Not sure if needed
*
*  MiddleWare (Multi Thread i guess)[_]
*
*  Radix tree routing[_]
*
*/
