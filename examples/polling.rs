use std::{env, time::Duration};

use tgbotool::{
    client::Client,
    methods::get_updates::{GetUpdates, GetUpdatesBuilder},
};
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("TG_BOT_TOKEN")?;
    let client = Client::new(&token);
    // get first update id
    let updates = client.get_updates(GetUpdates::default()).await?;
    let mut update_id = updates.last().map(|u| u.update_id).unwrap_or_default();
    loop {
        let body = GetUpdatesBuilder::new().offset(update_id + 1).build();
        let updates = client.get_updates(body).await?;
        if let Some(update) = updates.last() {
            update_id = update.update_id;
        }
        let resp = serde_json::to_string_pretty(&updates)?;
        println!("{resp}");
        time::sleep(Duration::from_millis(200)).await;
    }
}
