pub mod content_types;
pub mod status_code;

use std::{collections::HashMap, io::Read};

use super::error::Error;
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
    pub async fn new(
        http_version: HttpVersion,
        status_code: HttpStatusCode,
        mut headers: HashMap<String, String>,
    ) -> Result<Response, Error> {
        let content = Self::handle_file().await?;
        let len = content.len();
        if len > 0 {
            headers.insert("Content-Length".to_string(), len.to_string());
        }

        Ok(Response {
            http_version,
            status_code,
            headers,
            content,
        })
    }

    async fn handle_file() -> Result<String, Error> {
        let mut test_content = String::new();
        let mut file = std::fs::File::open("./src/http/response/test.html")?;
        file.read_to_string(&mut test_content)?;
        Ok(test_content)
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
