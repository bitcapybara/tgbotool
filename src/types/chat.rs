use serde_with::skip_serializing_none;

use super::message::Message;

/// This object represents a chat.
#[skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Chat {
    ///
    pub id: u64,
    ///
    pub photo: Option<ChatPhoto>,
    ///
    pub pinned_message: Option<Box<Message>>,
    ///
    pub message_auto_delete_time: Option<u64>,
    ///
    pub has_hidden_members: Option<bool>,
    ///
    pub has_protected_content: Option<bool>,
    ///
    #[serde(flatten)]
    #[serde(rename = "type")]
    pub chat_type: ChatType,
}

#[skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private {
        username: Option<String>,
        first_name: String,
        last_name: Option<String>,
        active_usernames: Option<Vec<String>>,
        emoji_status_custom_emoji_id: Option<String>,
        bio: Option<String>,
        has_private_forwards: Option<bool>,
        has_restricted_voice_and_video_messages: Option<bool>,
    },
    Group {
        title: String,
        description: Option<String>,
        invite_link: Option<String>,
        permissions: Option<ChatPermissions>,
    },
    Supergroup {
        title: String,
        username: Option<String>,
        is_forum: Option<bool>,
        active_usernames: Option<Vec<String>>,
        join_to_send_messages: Option<bool>,
        join_by_request: Option<bool>,
        description: Option<String>,
        invite_link: Option<String>,
        permissions: Option<ChatPermissions>,
        slow_mode_delay: Option<u64>,
        has_aggressive_anti_spam_enabled: Option<bool>,
        sticker_set_name: Option<String>,
        can_set_sticker_set: Option<bool>,
        linked_chat_id: Option<u64>,
        location: Option<ChatLocation>,
    },
    Channel {
        title: String,
        username: Option<String>,
        active_usernames: Option<Vec<String>>,
        description: Option<String>,
        invite_link: Option<String>,
        linked_chat_id: Option<u64>,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPhoto {
    pub small_file_id: String,
    pub small_file_unique_id: String,
    pub big_file_id: String,
    pub big_file_unique_id: String,
}

#[skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatPermissions {
    pub can_send_messages: Option<bool>,
    pub can_send_audios: Option<bool>,
    pub can_send_documents: Option<bool>,
    pub can_send_photos: Option<bool>,
    pub can_send_videos: Option<bool>,
    pub can_send_videos_notes: Option<bool>,
    pub can_send_voice_notes: Option<bool>,
    pub can_send_polls: Option<bool>,
    pub can_send_other_messages: Option<bool>,
    pub can_add_web_page_previews: Option<bool>,
    pub can_change_info: Option<bool>,
    pub can_invite_users: Option<bool>,
    pub can_pin_messages: Option<bool>,
    pub can_manage_topics: Option<bool>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatLocation {}
