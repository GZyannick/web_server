pub enum ContentType {
    Html,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ct = match self {
            Self::Html => "text/html",
        };

        write!(f, "{}", ct)
    }
}
