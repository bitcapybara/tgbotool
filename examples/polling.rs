use std::{env, time::Duration};

use tgbotool::{
    client::Client,
    methods::{
        get_updates::{GetUpdates, GetUpdatesBuilder},
        send_media_group::{InputFile, InputMediaPhotoBuilder, SendMediaGroupBuilder},
        ChatId, SendFile,
    },
};
use tokio::{fs, io::AsyncReadExt, time};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("TG_BOT_TOKEN")?;
    let client = Client::new(&token);
    let mut capy_bytes = Vec::new();
    fs::File::open("/home/yanyuxing/downloads/capybara.jpg")
        .await?
        .read_to_end(&mut capy_bytes)
        .await?;
    let mut bash_bytes = Vec::new();
    fs::File::open("/home/yanyuxing/downloads/gitbash.png")
        .await?
        .read_to_end(&mut bash_bytes)
        .await?;
    let group = SendMediaGroupBuilder::new(
        ChatId::Chat(5990504871),
        vec![
            InputFile::Photo(
                InputMediaPhotoBuilder::new(SendFile::upload("capybara", capy_bytes)).build(),
            ),
            InputFile::Photo(
                InputMediaPhotoBuilder::new(SendFile::upload("bash", bash_bytes)).build(),
            ),
        ],
    )
    .build();
    client.send_media_group(group).await?;

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
