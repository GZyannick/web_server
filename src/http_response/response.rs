use crate::http_parser::version::HttpVersion;
pub struct Response {
    http_version: HttpVersion,
    status_code: HttpStatusCode,
    content_type: String,
}
