use regex::Regex;

pub enum Token {
    Post(String, String),
    Fix(String, String),
    None,
}
impl Token {
    pub fn parse(s: String) -> Self {
        let re = Regex::new(r#"^(\w+)(?:\(([\w-]*)\))?:\s(.+)$"#).unwrap();
        let result = re.captures(&s).unwrap();
        let token = result.get(1).unwrap().as_str();
        match token {
            "post" | "upload" => Self::Post(
                result.get(2).unwrap().as_str().to_string(),
                result.get(3).unwrap().as_str().to_string(),
            ),
            "fix" => Self::Fix(
                result.get(2).unwrap().as_str().to_string(),
                result.get(3).unwrap().as_str().to_string(),
            ),
            _ => Self::None,
        }
    }
}
