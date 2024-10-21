use super::content_types::ContentType;
use super::status_code::HttpStatusCode;
use crate::http_parser::version::HttpVersion;

pub struct Response {
    http_version: HttpVersion,
    status_code: HttpStatusCode,
    content_type: ContentType,
    content_length: usize,
    content: String,
}

impl Response {
    pub async fn new() -> Response {
        let test_content = String::from("<h1>Hello World</h1>");
        let len = test_content.len();
        Response {
            http_version: HttpVersion::Http1_1,
            status_code: HttpStatusCode::S200,
            content_type: ContentType::Html,
            content_length: len,
            content: test_content,
        }
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            // http_version, status_code, content_type, content_length, content,
            "{} {}\r\ncontent-type:{}\r\ncontent-length:{}\r\n{}",
            self.http_version,
            self.status_code,
            self.content_type,
            self.content_length,
            self.content,
        )
    }
}
