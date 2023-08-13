use axum::{debug_handler, routing::post, Json, Router};
use clap::Parser;
use heck::ToSnakeCase;
use tg_cuckoo_bot::{
    command::{self, BotCommand},
    types::update::Update,
};

#[derive(clap::Parser)]
struct Args {
    #[arg(short, long, default_value = "0.0.0.0", env = "TG_CUCKOO_BOT_IP")]
    ip: String,
    #[arg(short, long, default_value_t = 9077, env = "TG_CUCKOO_BOT_PORT")]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let app = Router::new().route("/", post(process_webhook));

    let addr = format!("{}:{}", args.ip, args.port).parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[debug_handler]
async fn process_webhook(Json(update): Json<Update>) {
    // match serde_json::from_str::<Update>(&request) {
    //     Ok(update) => {
    //         println!("{:?}", update.update_id);
    //     }
    //     Err(e) => {
    //         println!("{e}")
    //     }
    // };
    println!("{}", update.update_id)
}

// #[derive(BotCommand)]
// #[command(bot_name = "cuckoo", rename_rule = "snake_case")]
pub enum Command {
    // #[command(rename = "/help")]
    Help,
    Username(String),
    UsernameAndAge { username: String, age: u8 },
}

impl BotCommand for Command {
    fn bot_name() -> String {
        "yx_cuckoo_bot".to_string()
    }

    fn parse(message: &str) -> Result<Self, command::Error> {
        let mut words = message.split_ascii_whitespace();
        let command = {
            let mut command_and_bot_name =
                words.next().ok_or(command::Error::TooFewArgs)?.split('@');
            let command = command_and_bot_name
                .next()
                .ok_or(command::Error::TooFewArgs)?;
            if let Some(bot_name) = command_and_bot_name.next() {
                if bot_name != Self::bot_name() {
                    return Err(command::Error::WrongBotName);
                }
            }
            command
        };

        match command.to_snake_case().as_str() {
            "/help" => {
                if words.next().is_some() {
                    return Err(command::Error::TooManyArgs);
                }
                Ok(Self::Help)
            }
            "/username" => {
                let arg = words.next().ok_or(command::Error::TooFewArgs)?;
                if words.next().is_some() {
                    return Err(command::Error::TooManyArgs);
                }
                Ok(Self::Username(arg.to_string()))
            }
            "/username_and_age" => {
                let username = words
                    .next()
                    .ok_or(command::Error::TooFewArgs)?
                    .parse::<String>()
                    .map_err(|e| command::Error::ParseError(e.to_string()))?;
                let age = words
                    .next()
                    .ok_or(command::Error::TooFewArgs)?
                    .parse::<u8>()
                    .map_err(|e| command::Error::ParseError(e.to_string()))?;
                if words.next().is_some() {
                    return Err(command::Error::TooManyArgs);
                }
                Ok(Self::UsernameAndAge { username, age })
            }
            _ => Err(command::Error::UnknownCmd),
        }
    }
}
