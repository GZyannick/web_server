use std::{collections::HashMap, u8};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[derive(Debug)]
enum HttpVersion {
    Http1_1,
}

impl From<&String> for HttpVersion {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "HTTP/1.1" => HttpVersion::Http1_1,
            _ => panic!("Cannot handle this http version"),
        }
    }
}

#[derive(Debug)]
enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    PUT,
}

impl From<&String> for Method {
    fn from(value: &String) -> Method {
        match value.as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            "PUT" => Method::PUT,
            _ => panic!("Get an invalide method parameter"),
        }
    }
}

//res: GET / HTTP/1.1   retrieve jusqua \n est split en trois
#[derive(Debug)]
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
        let mut split_input: Vec<String> = vec![];
        let mut is_first_line = true;

        for char in reader.iter() {
            match *char {
                // split method, uri and HttpVersion on the first line
                b' ' if is_first_line => {
                    if buffer.len() > 0 {
                        split_input.push(
                            String::from_utf8(buffer.clone()).expect("Contain invalide utf8"),
                        );
                        buffer.clear();
                    }
                }
                // handle new line
                b'\n' => {
                    // Check buffer length before push due to \r\n\r\n at the of the http request
                    if buffer.len() > 0 {
                        split_input.push(
                            String::from_utf8(buffer.clone()).expect("Contain invalide utf8"),
                        );
                        buffer.clear();
                    }
                    if is_first_line {
                        is_first_line = false;
                    }
                }

                // end of the buffer
                b'\0' => break,
                // dont push \r and \n
                b'\r' => (),
                _ => {
                    buffer.push(*char);
                }
            }
        }

        let mut split_input_iter = split_input.iter();
        let method: Method = split_input_iter.next().unwrap().into();
        let dirty_uri = split_input_iter.next().unwrap().to_owned();
        let version: HttpVersion = split_input_iter.next().unwrap().into();

        let mut headers: HashMap<String, String> = HashMap::new();
        for header in split_input_iter {
            let (k, v) = header.split_once(":").unwrap();
            headers.insert(k.into(), v.into());
        }

        let req = Request {
            method,
            uri: dirty_uri,
            version,
            headers,
            path_params: HashMap::new(),
            query_params: HashMap::new(),
        };

        //println!("{:#?}", req);

        Ok(req)
    }
}

async fn process_socket(socket: &mut tokio::net::TcpStream) {
    let mut buffer = [0; 1024];
    socket.read(&mut buffer).await.unwrap();
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
