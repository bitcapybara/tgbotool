#[derive(Default, serde::Serialize, tgbotool_derive::TgMethod, tgbotool_derive::Builder)]
pub struct GetUpdates {
    offset: Option<u64>,
    limit: Option<u8>,
    timeout: Option<u16>,
    allowed_updates: Option<Vec<String>>,
}
