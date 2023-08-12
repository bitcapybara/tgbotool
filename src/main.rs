use axum::{debug_handler, routing::post, Json, Router};
use clap::Parser;
use types::update::Update;

pub mod types;

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
