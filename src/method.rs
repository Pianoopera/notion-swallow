#[derive(Debug, PartialEq, Clone)]
pub enum MethodArg {
    GET,
    POST,
    PATCH,
    DELETE,
}

impl MethodArg {
    pub fn fmt(&self) -> String {
        match self {
            MethodArg::GET => "GET".to_string(),
            MethodArg::POST => "POST".to_string(),
            MethodArg::PATCH => "PATCH".to_string(),
            MethodArg::DELETE => "DELETE".to_string(),
        }
    }
    pub fn match_method(matches: &clap::ArgMatches) -> MethodArg {
        let method = matches.get_one::<String>("x")
            .map(String::as_str)
            .unwrap_or("GET");
        match method {
            "GET" => MethodArg::GET,
            "POST" => MethodArg::POST,
            "PATCH" => MethodArg::PATCH,
            "DELETE" => MethodArg::DELETE,
            _ => MethodArg::GET,
        }
    }
}