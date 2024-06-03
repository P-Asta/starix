use toml_reader::Config;
use webhook::client::WebhookClient;
mod toml_reader;
#[tokio::main]
async fn main() {
    let config =
        toml::from_str::<Config>(std::fs::read_to_string("Starix.toml").unwrap().as_str()).unwrap();
    let client: WebhookClient = WebhookClient::new(&config.webhook.uri);
    client
        .send(|message| {
            message.username("Thoo").embed(|embed| {
                embed
                    .title("Webhook")
                    .description("Hello, World!")
                    .field("name", "value", false)
            })
        })
        .await
        .unwrap();
}
