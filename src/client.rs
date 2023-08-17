use std::sync::Arc;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    methods::{
        answer_callback_query::AnswerCallbackQuery, get_file::GetFile, get_updates::GetUpdates,
        send_message::SendMessage, send_photo::SendPhoto,
    },
    types::{message::Message, update::Update, File, PhotoSize},
};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Response error: {0}")]
    Response(String),
    #[error("I/O error: {0}")]
    Io(String),
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum TgResponse<T> {
    Ok(OkResponse<T>),
    Err(ErrResponse),
}

#[derive(serde::Deserialize)]
pub struct OkResponse<T> {
    ok: bool,
    result: T,
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
    bot_token: String,
    client: Arc<reqwest::Client>,
}

impl Client {
    pub fn new(bot_token: &str) -> Self {
        Self {
            tg_url: format!("https://api.telegram.org/bot{bot_token}"),
            client: Arc::new(reqwest::Client::new()),
            bot_token: bot_token.to_owned(),
        }
    }
    pub(crate) async fn send_ok<T>(&self, method: &str, body: T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let url = format!("{}/{method}", self.tg_url);
        let raw_resp = self.client.post(url).json(&body).send().await?;
        let status = raw_resp.status();
        if !status.is_success() {
            let ErrResponse {
                error_code,
                description,
                ..
            } = raw_resp.json::<ErrResponse>().await?;
            return Err(Error::Response(format!(
                "status: {}: {} {}",
                status, error_code, description
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
        let raw_resp = self.client.post(url).json(&body).send().await?;
        self.get_response(raw_resp).await
    }

    async fn get_response<R>(&self, resp: reqwest::Response) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let status = resp.status();
        let resp = resp.json::<TgResponse<R>>().await?;
        match resp {
            TgResponse::Ok(OkResponse { result, .. }) => Ok(result),
            TgResponse::Err(ErrResponse {
                error_code,
                description,
                ..
            }) => Err(Error::Response(format!(
                "status: {}: {} {}",
                status, error_code, description
            ))),
        }
    }

    pub async fn send_message(&self, message: SendMessage) -> Result<()> {
        self.send_ok("sendMessage", message).await
    }

    pub async fn answer_callback_query(&self, message: AnswerCallbackQuery) -> Result<()> {
        self.send_ok("answerCallbackQuery", message).await
    }

    pub async fn send_photo(&self, photo: SendPhoto) -> Result<Vec<PhotoSize>> {
        let url = format!("{}/sendPhoto", self.tg_url);
        let mut req_builder = self.client.post(url);
        if photo.is_multipart() {
            req_builder = req_builder.multipart(photo.try_into().unwrap());
        } else {
            req_builder = req_builder.json(&photo);
        }
        let resp = req_builder.send().await?;
        let message = self.get_response::<Message>(resp).await?;
        Ok(message.photo.unwrap_or_default())
    }

    pub async fn get_updates(&self, body: GetUpdates) -> Result<Vec<Update>> {
        self.send("getUpdates", body).await
    }

    pub async fn get_file(&self, body: GetFile) -> Result<Option<Vec<u8>>> {
        let file: File = self.send("getFile", body).await?;
        let Some(file_path) = file.file_path else {
            return Ok(None);
        };
        let download_url = format!(
            "https://api.telegram.org/file/bot{}/{}",
            self.bot_token, file_path
        );
        let resp = self.client.get(download_url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(Error::Response(format!("download file error: {}", status)));
        }

        Ok(Some(resp.bytes().await?.to_vec()))
    }
}
