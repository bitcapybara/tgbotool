use axum::{debug_handler, extract::State, routing::post, Json, Router};
use clap::Parser;
use tgbotool::{
    client::Client,
    methods::{
        answer_callback_query::AnswerCallbackQueryBuilder, send_message::SendMessageBuilder,
        ChatId, ReplyMarkup,
    },
    types::{
        update::{Update, UpdateType},
        InlineKeyboardButtonBuilder,
    },
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
#[command(bot_name = "yx_cuckoo_bot", rename_rule = "snake_case")]
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
    match &update.update_type {
        UpdateType::Message(msg) => {
            let Some(text) = &msg.text else { return };
            let chat_id = ChatId::Chat(msg.chat.id);
            let send_message = SendMessageBuilder::new(chat_id, text)
                .reply_markup(ReplyMarkup::inline_keyboard(vec![vec![
                    InlineKeyboardButtonBuilder::new("取消")
                        .callback_data("cancel")
                        .build(),
                    InlineKeyboardButtonBuilder::new("确定")
                        .callback_data("ok")
                        .build(),
                ]]))
                .build();
            if let Err(e) = client.send_message(send_message).await {
                println!("send_message error: {e}");
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
