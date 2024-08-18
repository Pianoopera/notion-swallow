#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
}

impl Method {
    pub fn fmt(&self) -> String {
        match self {
            Method::GET => "GET".to_string(),
            Method::POST => "POST".to_string(),
            Method::PATCH => "PATCH".to_string(),
            Method::DELETE => "DELETE".to_string(),
        }
    }
    pub fn match_method(method: &str) -> Method {
        match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            _ => Method::GET,
        }
    }
}