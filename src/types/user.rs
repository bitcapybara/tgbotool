use serde_with::skip_serializing_none;

/// This object represents a Telegram user or bot
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: u64,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User's or bot's first name
    pub first_name: String,
    /// Optional. User's or bot's last name
    pub last_name: Option<String>,
    /// Optional. User's or bot's username
    pub username: Option<String>,
    /// Optional. IETF language tag of the user's language
    pub language_code: Option<String>,
    /// Optional. True, if this user is a Telegram Premium user
    pub is_premium: Option<bool>,
    /// Optional. True, if this user added the bot to the attachment menu
    pub added_to_attachment_menu: Option<bool>,
    /// Optional. True, if the bot can be invited to groups. Returned only in `getMe`.
    pub can_join_groups: Option<bool>,
    /// Optional. True, if privacy mode is disabled for the bot. Returned only in `getMe`.
    pub can_read_all_group_messages: Option<bool>,
    /// Optional. True, if the bot supports inline queries. Returned only in `getMe`.
    pub supports_inline_queries: Option<bool>,
}
