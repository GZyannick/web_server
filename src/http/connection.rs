use super::{error::Error, request::Request, response::Response, router::Router};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub struct Connection {
    req: Request,
    socket: tokio::net::TcpStream,
}

impl Connection {
    pub async fn new(mut socket: tokio::net::TcpStream) -> Result<Connection, Error> {
        let mut buffer = [0; 1024];
        socket.read(&mut buffer).await?;
        let req = Request::new(&mut buffer).await?;
        Ok(Connection { req, socket })
    }

    pub async fn respond<'a>(&mut self, router: &'a Router) -> Result<(), Error> {
        match router.root.find(&self.req.uri) {
            Some((res, path_params)) => {
                // add path_parameter to request
                for (k, v) in path_params.params_iter() {
                    self.req.path_params.insert(k.to_string(), v.to_string());
                }
                self.socket.write_all(format!("{}", res).as_bytes()).await?;
            }
            None => {
                let not_found = Response::not_found().await;
                self.socket
                    .write_all(format!("{}", not_found).as_bytes())
                    .await?;
            }
        };

        self.socket.flush().await?;

        Ok(())
    }
}
