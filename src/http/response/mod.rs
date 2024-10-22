pub mod content_types;
pub mod status_code;

use std::{collections::HashMap, io::Read};

use super::version::HttpVersion;
//use content_types::ContentType;
use status_code::HttpStatusCode;

pub struct Response {
    http_version: HttpVersion,
    status_code: HttpStatusCode,
    headers: HashMap<String, String>,
    content: String,
}

impl Response {
    pub async fn new() -> Response {
        let mut test_content = String::new();
        let mut file = std::fs::File::open("./test.html").unwrap();
        file.read_to_string(&mut test_content).unwrap();
        //let test_content = String::from("<h1>Hello World</h1>");
        let len = test_content.len();
        let mut headers: HashMap<String, String> = HashMap::new();
        if len > 0 {
            headers.insert("Content-Length".to_string(), len.to_string());
        }
        // Exemple
        Response {
            http_version: HttpVersion::Http1_1,
            status_code: HttpStatusCode::S200,
            headers: HashMap::new(),
            content: test_content,
        }
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut headers_buffer = String::new();
        if self.headers.len() > 0 {
            for (k, v) in self.headers.iter() {
                let header = format!("{}: {}\r\n", k, v);
                headers_buffer.push_str(&header.as_str());
            }
        }

        write!(
            f,
            "{version} {status}\r\n{headers}\r\n{content}",
            version = self.http_version,
            status = self.status_code,
            headers = headers_buffer,
            content = self.content,
        )
    }
}