use serde_with::skip_serializing_none;

pub struct AnswerCallbackQueryBuilder {
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<u64>,
}

impl AnswerCallbackQueryBuilder {
    pub fn new(callback_query_id: &str) -> Self {
        Self {
            callback_query_id: callback_query_id.to_owned(),
            text: None,
            show_alert: None,
            url: None,
            cache_time: None,
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_owned());
        self
    }

    pub fn show_alert(mut self, show_alert: bool) -> Self {
        self.show_alert = Some(show_alert);
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }

    pub fn cache_time(mut self, cache_time: u64) -> Self {
        self.cache_time = Some(cache_time);
        self
    }

    pub fn build(self) -> AnswerCallbackQuery {
        AnswerCallbackQuery {
            callback_query_id: self.callback_query_id,
            text: self.text,
            show_alert: self.show_alert,
            url: self.url,
            cache_time: self.cache_time,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, serde::Serialize)]
pub struct AnswerCallbackQuery {
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<u64>,
}
