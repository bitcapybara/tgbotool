use std::{env, time::Duration};

use tgbotool::{
    client::Client,
    methods::{
        get_updates::{GetUpdates, GetUpdatesBuilder},
        send_media::SendVideoBuilder,
        ChatId, SendFile, UploadFile,
    },
    types::{message::Message, update::Update},
};
use tokio::{fs, io::AsyncReadExt, time};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("TG_BOT_TOKEN")?;
    let client = Client::new(&token);
    let mut capy_bytes = Vec::new();
    fs::File::open("/home/yanyuxing/downloads/bara.mp4")
        .await?
        .read_to_end(&mut capy_bytes)
        .await?;
    let mut bash_bytes = Vec::new();
    fs::File::open("/home/yanyuxing/downloads/gitbash.png")
        .await?
        .read_to_end(&mut bash_bytes)
        .await?;
    let video = SendVideoBuilder::new(
        ChatId::Chat(5990504871),
        SendFile::upload("capybara", capy_bytes),
    )
    .thumbnail(UploadFile::new("thsumb", bash_bytes))
    .build();
    let msg: Message = client.send_media(video).await?;
    println!("{}", serde_json::to_string(&msg)?);

    // get first update id
    let updates: Vec<Update> = client.send(GetUpdates::default()).await?;
    let mut update_id = updates.last().map(|u| u.update_id).unwrap_or_default();
    loop {
        let body = GetUpdatesBuilder::new().offset(update_id + 1).build();
        let updates: Vec<Update> = client.send(body).await?;
        if let Some(update) = updates.last() {
            update_id = update.update_id;
        }
        let resp = serde_json::to_string_pretty(&updates)?;
        println!("{resp}");
        time::sleep(Duration::from_millis(200)).await;
    }
}
