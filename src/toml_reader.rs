#[derive(serde::Deserialize, Debug)]
pub struct Config {
    webhook: Webhook,
}

#[derive(serde::Deserialize, Debug)]
pub struct Webhook {
    uri: String,
}
