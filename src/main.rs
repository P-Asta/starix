use config::Config;
use std::{io::Read, process::Command};
use token::Token;
use webhook::client::WebhookClient;
mod config;
mod token;
#[tokio::main]
async fn main() {
    let uri = std::env::var("URI").unwrap();
    let client: WebhookClient = WebhookClient::new(&uri);
    let send_config =
        toml::from_str::<Config>(&std::fs::read_to_string("Starix.toml").unwrap_or("".to_string()))
            .unwrap_or(Config {
                url: "https://github.com/5-23".to_string(),
                thumb: "".to_string(),
                name: "Asta".to_string(),
                post: config::SubConfig {
                    content: "syntex error".to_string(),
                    color: "#ff0000".to_string(),
                },
                fix: config::SubConfig {
                    content: "syntex error".to_string(),
                    color: "#ff0000".to_string(),
                },
            });

    let comment = Command::new("git")
        .arg("log")
        .arg("--format=%s")
        .arg("-1")
        .output()
        .unwrap();
    let commit_title = String::from_utf8_lossy(&comment.stdout);
    match Token::parse(commit_title.to_string()) {
        Token::Post(identifire, description) => {
            let config_name = send_config.name.replace("{starix.id}", &identifire);
            let config_url = send_config.url.replace("{starix.id}", &identifire);
            let config_thumb = send_config.thumb.replace("{starix.id}", &identifire);
            let config_content = send_config.post.content.replace("{starix.id}", &identifire);
            client
                .send(|message| {
                    message
                        .username(&config_name)
                        .content(&config_content)
                        .embed(|embed| {
                            let embed = embed
                                .title(&identifire)
                                .description(&description)
                                .url(&config_url)
                                .color(&hstr_to_dstr(send_config.post.color.clone()));
                            if config_thumb != "" {
                                embed.image(&config_thumb)
                            } else {
                                embed
                            }
                        })
                })
                .await
                .unwrap();
        }
        Token::Fix(identifire, description) => {
            let config_name = send_config.name.replace("{starix.id}", &identifire);
            let config_url = send_config.url.replace("{starix.id}", &identifire);
            let config_thumb = send_config.thumb.replace("{starix.id}", &identifire);
            let config_content = send_config.fix.content.replace("{starix.id}", &identifire);
            client
                .send(|message| {
                    message
                        .username(&config_name)
                        .content(&config_content)
                        .embed(|embed| {
                            let embed = embed
                                .title(&identifire)
                                .description(&description)
                                .url(&config_url)
                                .color(&hstr_to_dstr(send_config.fix.color.clone()));

                            if config_thumb != "" {
                                embed.image(&config_thumb)
                            } else {
                                embed
                            }
                        })
                })
                .await
                .unwrap();
        }
        _ => {}
    }
}

fn hstr_to_dstr(s: String) -> String {
    usize::from_str_radix(&s.replace("#", ""), 16)
        .unwrap()
        .to_string()
}
