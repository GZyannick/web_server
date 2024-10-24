#[derive(Debug, Clone, Copy)]
pub enum HttpStatusCode {
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
    NotImplemented,
    BadGateway,
    HTTPVersionNotSupported,
}

impl std::fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::Created => write!(f, "201 Created"),
            Self::Accepted => write!(f, "202 Accepted"),
            Self::NoContent => write!(f, "204 No Content"),
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::Unauthorized => write!(f, "401 Unauthorized"),
            Self::Forbidden => write!(f, "403 Forbidden"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
            Self::NotImplemented => write!(f, "501 Not Implemented"),
            Self::BadGateway => write!(f, "502 Bad Gateway"),
            Self::HTTPVersionNotSupported => write!(f, "505 HTTP Version Not Supported"),
        }
    }
}
