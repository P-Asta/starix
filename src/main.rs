use toml_reader::Config;
use webhook::client::WebhookClient;
mod toml_reader;
#[tokio::main]
async fn main() {
    let uri = std::env::var("URI").unwrap();
    let client: WebhookClient = WebhookClient::new(&uri);
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
