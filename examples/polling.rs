use std::time::Duration;

use tgbotool::{methods::get_updates::GetUpdatesBuilder, types::update::Update};

#[derive(tgbotool_derive::BotCommand)]
enum Command {
    Start,
    AddPlan {
        _cron: String,
        _time_zone: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = std::env::var("TG_BOT_TOKEN")?;
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(40))
        .connect_timeout(Duration::from_secs(5))
        .build()?;
    let client = tgbotool::Client::new(&token, http_client);
    // get first update id
    let mut update_id = 0;
    loop {
        let body = GetUpdatesBuilder::new()
            .offset(update_id + 1)
            .timeout(30)
            .build();
        let updates: Vec<Update> = client.send(body).await?;
        if let Some(update) = updates.last() {
            update_id = update.update_id;
        }
        let resp = serde_json::to_string_pretty(&updates)?;
        println!("{resp}");
    }
}
