use std::sync::Arc;

use crate::{
    methods::{get_file::GetFile, TgMethod},
    types::File,
};

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    Request(String),
    #[error("Response error: {0}")]
    Response(String),
    #[error("I/O error: {0}")]
    Io(String),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Request(value.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Request(value.to_string())
    }
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
    pub fn new(bot_token: &str, http_client: reqwest::Client) -> Self {
        Self {
            tg_url: format!("https://api.telegram.org/bot{bot_token}"),
            client: Arc::new(http_client),
            bot_token: bot_token.to_owned(),
        }
    }
    pub async fn send_ok<T>(&self, body: T) -> Result<(), Error>
    where
        T: TgMethod,
        T: TgMethod,
    {
        let resp = self.build_request(body).await?;
        self.get_ok_response(resp).await
    }
    pub async fn send_media_ok<T>(&self, body: T) -> Result<(), Error>
    where
        T: TgMethod + TryInto<reqwest::multipart::Form, Error = serde_json::Error>,
        T: TgMethod,
    {
        let resp = self.build_media_request(body).await?;
        self.get_ok_response(resp).await
    }

    pub async fn send<T, R>(&self, body: T) -> Result<R, Error>
    where
        T: TgMethod,
        R: serde::de::DeserializeOwned,
    {
        let resp = self.build_request(body).await?;
        self.get_response(resp).await
    }

    pub async fn send_media<T, R>(&self, body: T) -> Result<R, Error>
    where
        T: TgMethod + TryInto<reqwest::multipart::Form, Error = serde_json::Error>,
        R: serde::de::DeserializeOwned,
    {
        let resp = self.build_media_request(body).await?;
        self.get_response(resp).await
    }

    async fn build_request<T>(&self, body: T) -> Result<reqwest::Response>
    where
        T: TgMethod,
    {
        let url = format!("{}/{}", self.tg_url, T::method_name());
        Ok(self.client.post(url).json(&body).send().await?)
    }

    async fn build_media_request<T>(&self, body: T) -> Result<reqwest::Response>
    where
        T: TgMethod + TryInto<reqwest::multipart::Form, Error = serde_json::Error>,
    {
        let url = format!("{}/{}", self.tg_url, T::method_name());
        let mut req_builder = self.client.post(url);
        if body.is_multipart() {
            req_builder = req_builder.multipart(body.try_into()?);
        } else {
            req_builder = req_builder.json(&body);
        }
        Ok(req_builder.send().await?)
    }

    async fn get_response<R>(&self, resp: reqwest::Response) -> Result<R, Error>
    where
        R: serde::de::DeserializeOwned,
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

    async fn get_ok_response(&self, resp: reqwest::Response) -> Result<(), Error> {
        let status = resp.status();
        if !status.is_success() {
            let ErrResponse {
                error_code,
                description,
                ..
            } = resp.json::<ErrResponse>().await?;
            return Err(Error::Response(format!(
                "status: {}: {} {}",
                status, error_code, description
            )));
        }
        Ok(())
    }

    pub async fn get_file(&self, body: GetFile) -> Result<Option<Vec<u8>>> {
        let file: File = self.send(body).await?;
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
