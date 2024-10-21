use crate::http_parser::version::HttpVersion;

enum HttpStatusCode {
    S200,
    S404,
    S500,
}

impl std::fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::S200 => write!(f, "200 Ok\r\n"),
        }
    }
}

struct Response {
    http_version: HttpVersion,
    status_code: HttpStatusCode,
    content_type: String,
}
