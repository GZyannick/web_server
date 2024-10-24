mod http;
use http::connection::Connection;
use http::error::Error;
use http::response::status_code::HttpStatusCode;
use http::response::Response;
use http::router::Router;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut router = Router::new(
        "/",
        Response::new(HttpStatusCode::Ok, HashMap::new(), "Home").await?,
    )
    .await?;

    let _ = router.insert(
        "/hello",
        Response::new(HttpStatusCode::Ok, HashMap::new(), "Hello World").await?,
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        let mut conn = Connection::new(socket).await?;
        conn.respond(&router).await?;
    }
}
