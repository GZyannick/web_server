use core::fmt;
use std::{collections::HashMap, io::BufRead, u8};

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

enum HttpVersion {
    Http1_1,
}

enum ParsingError {
    Method,
    //TokioErr(tokio::io::Error),
    //StdErr(tokio::io::Error),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::Method => write!(f, "Cannot find expected Method"),
            //ParsingError::TokioErr(e) => write!(f, "{e}"),
            //ParsingError::StdErr(e) => write!(f, "{e}"),
        }
    }
}

enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

impl Method {
    pub fn new(method: &String) -> Result<Method, ParsingError> {
        let method = match method.as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            "PUT" => Self::PUT,
            _ => return Err(ParsingError::Method),
        };
        Ok(method)
    }
}

//res: GET / HTTP/1.1   retrieve jusqua \n est split en trois
struct Request {
    method: Method,                        // GET, POST...
    uri: String,                           // TODO! a determiner si cest tout ou juste path
    version: HttpVersion,                  // HTTP/1.1 HTTP/2.0 HTTP/3.0
    headers: HashMap<String, String>,      // Accept-Encoding: ......
    path_params: HashMap<String, String>,  // /user/bob ou /user/{id}
    query_params: HashMap<String, String>, // /user?username:bob&country:france
}

impl Request {
    pub async fn new(reader: &mut [u8]) -> tokio::io::Result<Request> {
        let mut buffer: Vec<u8> = vec![];
        let mut split_input: Vec<Vec<u8>> = vec![];
        let mut is_first_line = true;

        let mut split_hash_map: HashMap<String, u8> = std::collections::HashMap::new();

        for char in reader.iter() {
            match *char {
                b'\n' => {
                    // if this is the first line split it to get the method, uri and http version
                    //if is_first_line && buffer.len() > 0 {
                    //    let split = buffer.split(b' ');
                    //}
                    // Check buffer length before push due to \r\n\r\n at the of the http request
                    else if buffer.len() > 0 {
                        split_input.push(buffer.clone());
                        buffer.clear();
                    }
                }
                b'\0' => break,
                b'\r' => (),

                _ => {
                    buffer.push(*char);
                }
            }
        }

        for input in split_input {
            let res = String::from_utf8(input).unwrap();
            println!("{res}");
        }

        todo!()
    }
}

async fn process_socket(socket: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await.unwrap();

    //let res = std::str::from_utf8(&buffer).unwrap();
    //println!("{:#?}", res);
    Request::new(&mut buffer).await.unwrap();
    let _ = socket.write_all(b"Hello World");
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        process_socket(&mut socket).await;
    }
}

//exemple de requete avec query_params
// 127.0.0.1:8080/user?username=eric&country=fr
// resultat de la requete
//res: GET /user?username=eric&country=fr HTTP/1.1
//...........

//exemple de requete avec path_params
// 127.0.0.1:8080/user/eric
// resultat de la requete
//res: GET /user/eric HTTP/1.1
//...........

// Requete de base juste en appelant l'ip
//
//res: GET / HTTP/1.1
//Host: 127.0.0.1:8080
//User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:131.0) Gecko/20100101 Firefox/131.0
//Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/png,image/svg+xml,*/*;q=0.8
//Accept-Language: en-GB,en;q=0.5
//Accept-Encoding: gzip, deflate, br, zstd
//Connection: keep-alive
//Upgrade-Insecure-Requests: 1
//Sec-Fetch-Dest: document
//Sec-Fetch-Mode: navigate
//Sec-Fetch-Site: none
//Sec-Fetch-User: ?1
//Priority: u=0, i
