#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub webhook: Webhook,
}

#[derive(serde::Deserialize, Debug)]
pub struct Webhook {
    pub uri: String,
}
