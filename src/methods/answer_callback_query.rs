use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, serde::Serialize, tgbotool_derive::Builder)]
pub struct AnswerCallbackQuery {
    callback_query_id: String,
    text: Option<String>,
    show_alert: Option<bool>,
    url: Option<String>,
    cache_time: Option<u64>,
}
