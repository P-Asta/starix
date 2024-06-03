#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub url: String,
    pub thumb: String,
    pub name: String,
    pub avatar: String,
    pub post: SubConfig,
    pub fix: SubConfig,
}
#[derive(Debug, serde::Deserialize)]
pub struct SubConfig {
    pub content: String,
    pub color: String,
}
