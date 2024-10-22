#[derive(Debug)]
pub enum Method {
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
