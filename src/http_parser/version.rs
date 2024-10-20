#[derive(Debug)]
pub enum HttpVersion {
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

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http1_1 => write!(f, "HTTP/1.1"),
        }
    }
}
