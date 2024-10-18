//res: GET / HTTP/1.1   retrieve jusqua \n est split en trois

use crate::http_parser::{method::Method, version::HttpVersion};
use std::{collections::HashMap, u8};
#[derive(Debug)]
pub struct Request {
    pub method: Method,                        // GET, POST...
    pub uri: String,                           // TODO! a determiner si cest tout ou juste path
    pub version: HttpVersion,                  // HTTP/1.1 HTTP/2.0 HTTP/3.0
    pub headers: HashMap<String, String>,      // Accept-Encoding: ......
    pub path_params: HashMap<String, String>,  // /user/bob ou /user/{id}
    pub query_params: HashMap<String, String>, // /user?username:bob&country:france
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
        let uri = split_input_iter.next().unwrap().to_owned();

        // get query params if exist
        let query_params = match uri.contains("?") {
            true => {
                let mut query_params: HashMap<String, String> = HashMap::new();
                let (_, params) = uri.split_once("?").unwrap();

                let params = params.split("&");
                for param in params {
                    let (k, v) = param.split_once("=").unwrap();
                    query_params.insert(k.to_string(), v.to_string());
                }
                query_params
            }
            false => HashMap::new(),
        };

        let version: HttpVersion = split_input_iter.next().unwrap().into();

        let mut headers: HashMap<String, String> = HashMap::new();
        for header in split_input_iter {
            let (k, v) = header.split_once(":").unwrap();
            headers.insert(k.into(), v.into());
        }

        let req = Request {
            method,
            uri,
            version,
            headers,
            path_params: HashMap::new(),
            query_params,
        };

        println!("{:#?}", req);

        Ok(req)
    }
}
