use axum::{debug_handler, extract::State, routing::post, Json, Router};
use clap::Parser;
use tg_cuckoo_bot::{
    client::Client,
    methods::message::{ChatId, SendMessageBuilder},
    types::update::{Update, UpdateType},
    BotCommand,
};

#[derive(clap::Parser)]
struct Args {
    #[arg(env = "TG_BOT_TOKEN")]
    bot_token: String,
    #[arg(short, long, default_value = "0.0.0.0", env = "TG_CUCKOO_BOT_IP")]
    ip: String,
    #[arg(short, long, default_value_t = 9077, env = "TG_CUCKOO_BOT_PORT")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = Client::new(&args.bot_token);
    let app = Router::new()
        .route("/", post(process_webhook))
        .with_state(client);

    let addr = format!("{}:{}", args.ip, args.port).parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, BotCommand)]
#[command(rename_rule = "snake_case")]
pub enum Cuckoo {
    #[command(rename = "/help")]
    Help,
    Username(String, u8),
    UsernameAndAge {
        username: String,
        age: u8,
    },
}

#[debug_handler]
async fn process_webhook(State(client): State<Client>, Json(update): Json<Update>) {
    // match serde_json::from_str::<Update>(&request) {
    //     Ok(update) => {
    //         println!("{:?}", update.update_id);
    //     }
    //     Err(e) => {
    //         println!("{e}")
    //     }
    // };
    if let UpdateType::Message(msg) = &update.update_type {
        let send_message = SendMessageBuilder::new(ChatId::Chat(msg.chat.id), "aaa").build();
        if let Err(e) = client.send_message(send_message).await {
            println!("send_message error: {e}");
        }
    }
}
