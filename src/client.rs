use std::sync::Arc;

use serde::{de::DeserializeOwned, Serialize};

use crate::methods::{answer_callback_query::AnswerCallbackQuery, send_message::SendMessage};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Response error: {0}")]
    Response(String),
}

#[derive(serde::Deserialize)]
pub struct ErrResponse {
    ok: bool,
    error_code: u16,
    description: String,
}

#[derive(Clone)]
pub struct Client {
    tg_url: String,
    client: Arc<reqwest::Client>,
}

impl Client {
    pub fn new(bot_token: &str) -> Self {
        Self {
            tg_url: format!("https://api.telegram.org/bot{bot_token}"),
            client: Arc::new(reqwest::Client::new()),
        }
    }
    pub(crate) async fn send_ok<T>(&self, method: &str, body: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let url = format!("{}/{method}", self.tg_url);
        let resp = self.client.post(url).json(&body).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let err_resp = resp.json::<ErrResponse>().await?;
            return Err(Error::Response(format!(
                "tg response error: {} {}",
                err_resp.error_code, err_resp.description
            )));
        }
        Ok(())
    }

    pub(crate) async fn send<T, R>(&self, method: &str, body: T) -> Result<R, Error>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}/{method}", self.tg_url);
        let resp = self.client.post(url).json(&body).send().await?;
        let status = resp.status();
        if !status.is_success() {
            let err_resp = resp.json::<ErrResponse>().await?;
            return Err(Error::Response(format!(
                "tg response error: {} {}",
                err_resp.error_code, err_resp.description
            )));
        }
        Ok(resp.json().await?)
    }

    pub async fn send_message(&self, message: SendMessage) -> Result<()> {
        self.send_ok("sendMessage", message).await
    }

    pub async fn answer_callback_query(&self, message: AnswerCallbackQuery) -> Result<()> {
        self.send_ok("answerCallbackQuery", message).await
    }
}
