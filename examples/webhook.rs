use axum::{debug_handler, extract::State, routing::post, Json, Router};
use clap::Parser;
use tgbotool::{
    client::Client,
    methods::{
        answer_callback_query::AnswerCallbackQueryBuilder, send_media::SendPhotoBuilder, ChatId,
        SendFile,
    },
    types::update::{Update, UpdateType},
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

#[debug_handler]
async fn process_webhook(State(client): State<Client>, Json(update): Json<Update>) {
    match &update.update_type {
        UpdateType::Message(msg) => {
            let Some(_text) = &msg.text else { return };
            let chat_id = ChatId::Chat(msg.chat.id);
            println!("{}", msg.chat.id);
            let send_file = SendFile::id_or_url(
                "AgACAgUAAxkDAANNZNxAxTaHn7g_YEbJR1qg8t54TEUAAo61MRuDK-hW_Cn_fSdb_YIBAAMCAAN4AAMwBA"
            );
            let photo = SendPhotoBuilder::new(chat_id, send_file)
                .caption("send file #abc")
                .build();
            if let Err(e) = client.send_photo(photo).await {
                println!("send photo error: {e}")
            }
        }
        UpdateType::EditedMessage(_) => todo!(),
        UpdateType::ChannelPost(_) => todo!(),
        UpdateType::EditedChannelPost(_) => todo!(),
        UpdateType::InlineQuery(_) => todo!(),
        UpdateType::ChosenInlineResult(_) => todo!(),
        UpdateType::CallbackQuery(cq) => {
            println!("receive callback: {:?}", cq.data.as_ref());
            let mut answer_message = AnswerCallbackQueryBuilder::new(&cq.id);
            if let Some(data) = cq.data.as_ref() {
                answer_message = answer_message.text(data).show_alert(true);
            }
            if let Err(e) = client.answer_callback_query(answer_message.build()).await {
                println!("send callback answer error: {e}")
            }
        }
        UpdateType::ShippingQuery(_) => todo!(),
        UpdateType::PreCheckoutQuery(_) => todo!(),
        UpdateType::Poll(_) => todo!(),
        UpdateType::PollAnswer(_) => todo!(),
        UpdateType::MyChatMember(_) => todo!(),
        UpdateType::ChatMember(_) => todo!(),
        UpdateType::ChatJoinRequest(_) => todo!(),
    }
}
