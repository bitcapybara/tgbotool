use std::{iter::Peekable, ops::Range, str::Chars};

use serde_with::skip_serializing_none;

use super::{
    chat::Chat,
    game::Game,
    passport::PassportData,
    payment::{Invoice, SuccessfulPayment},
    Animation, Audio, ChatShared, Contact, Dice, Document, ForumTopicClosed, ForumTopicCreated,
    ForumTopicEdited, ForumTopicReopened, GeneralForumTopicHidden, GeneralForumTopicUnHidden,
    InlineKeyboardMarkup, Location, MessageAutoDeleteTimerChanged, PhotoSize, Poll,
    ProximityAlertTriggered, Sticker, User, UserShared, Venue, Video, VideoChatEnded,
    VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted, VideoNote, Voice,
    WebAppData, WriteAccessAllowed,
};

#[skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub message_id: u64,
    pub message_thread_id: Option<u64>,
    pub from: Option<User>,
    pub sender_chat: Option<Box<Chat>>,
    pub date: u64,
    pub chat: Box<Chat>,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<u64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<u64>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<u64>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub animation: Option<Animation>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub video_note: Option<VideoNote>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub has_media_spoiler: Option<bool>,
    pub contact: Option<Contact>,
    pub dice: Option<Dice>,
    pub game: Option<Game>,
    pub poll: Option<Poll>,
    pub venue: Option<Venue>,
    pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<PhotoSize>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<u64>,
    pub migrate_from_chat_id: Option<u64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub user_shared: Option<UserShared>,
    pub chat_shared: Option<ChatShared>,
    pub connected_website: Option<String>,
    pub write_access_allowd: Option<WriteAccessAllowed>,
    pub passport_data: Option<PassportData>,
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    pub forum_topic_created: Option<ForumTopicCreated>,
    pub forum_topic_edited: Option<ForumTopicEdited>,
    pub forum_topic_closed: Option<ForumTopicClosed>,
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    pub genenral_forum_topic_unhidden: Option<GeneralForumTopicUnHidden>,
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    pub video_chat_started: Option<VideoChatStarted>,
    pub video_chat_ended: Option<VideoChatEnded>,
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    pub web_app_data: Option<WebAppData>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, tgbotool_derive::Builder)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub entity_type: MessageEntityType,
    pub offset: usize,
    pub length: usize,
    pub url: Option<String>,
    pub user: Option<User>,
    pub language: Option<String>,
    pub custom_emoji_id: Option<String>,
}

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityType {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre,
    TextLink,
    TextMention,
    CustomEmoji,
}

#[derive(Debug, PartialEq)]
pub struct MessageEntityRef<'a> {
    message: &'a str,
    range: Range<usize>,
    entity_type: &'a MessageEntityType,
}

impl<'a> MessageEntityRef<'a> {
    fn copy(&self, range: Range<usize>) -> Self {
        Self {
            message: self.message,
            range,
            entity_type: self.entity_type,
        }
    }
}

struct EntityCharIter<'a> {
    chars: Peekable<Chars<'a>>,
    utf8_offset: usize,
    utf16_offset: usize,
    current: Option<char>,
}

impl<'a> EntityCharIter<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            chars: text.chars().peekable(),
            utf8_offset: 0,
            utf16_offset: 0,
            current: None,
        }
    }
}

impl<'a> Iterator for EntityCharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            self.utf8_offset += current.len_utf8();
            self.utf16_offset += current.len_utf16();
        }
        self.current = self.chars.peek().cloned();
        self.chars.next()
    }
}

pub fn parse_entities<'a>(
    text: &'a str,
    entities: &'a [MessageEntity],
) -> Vec<MessageEntityRef<'a>> {
    let mut res = Vec::with_capacity(entities.len());

    // utf-16 range
    let mut offsets = entities.iter().map(|e| MessageEntityRef {
        message: text,
        range: e.offset..e.offset + e.length,
        entity_type: &e.entity_type,
    });

    let mut chars = EntityCharIter::new(text);
    let Some(mut offset) = offsets.next() else {
        return res;
    };
    while chars.next().is_some() {
        if chars.utf16_offset >= offset.range.start {
            let finded_utf8_offset = chars.utf8_offset;
            // find offset
            while let Some(char) = chars.next() {
                if chars.utf16_offset + char.len_utf16() >= offset.range.end {
                    res.push(offset.copy(finded_utf8_offset..chars.utf8_offset + char.len_utf8()));
                    let Some(entity) = offsets.next() else {
                        return res;
                    };
                    offset = entity;
                    break;
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entities_0() {
        let msgs: Vec<MessageEntity> = serde_json::from_str(
            r#"
                [
                    {
                      "type": "strikethrough",
                      "offset": 0,
                      "length": 7
                    }
                ]
            "#,
        )
        .unwrap();
        assert_eq!(
            parse_entities("jkflsd;", &msgs)
                .iter()
                .filter_map(|e| e.message.get(e.range.clone()))
                .collect::<Vec<&str>>(),
            ["jkflsd;"]
        )
    }

    #[test]
    fn entities_1() {
        let msgs: Vec<MessageEntity> = serde_json::from_str(
            r#"
                [
                    {
                        "type": "url",
                        "offset": 3,
                        "length": 21
                    }
                ]
            "#,
        )
        .unwrap();
        assert_eq!(
            parse_entities("上周 https://www.baidu.com", &msgs)
                .iter()
                .filter_map(|e| e.message.get(e.range.clone()))
                .collect::<Vec<&str>>(),
            ["https://www.baidu.com"]
        )
    }

    #[test]
    fn entities_2() {
        let msgs: Vec<MessageEntity> = serde_json::from_str(
            r#"
                [
                    {
                        "type": "hashtag",
                        "offset": 2,
                        "length": 3
                    }
                ]
            "#,
        )
        .unwrap();
        assert_eq!(
            parse_entities("我 #上班 o", &msgs)
                .iter()
                .filter_map(|e| e.message.get(e.range.clone()))
                .collect::<Vec<&str>>(),
            ["#上班"]
        )
    }

    #[test]
    fn entities_3() {
        let msgs: Vec<MessageEntity> = serde_json::from_str(
            r#"
                [
                    {
                      "type": "hashtag",
                      "offset": 4,
                      "length": 3
                    },
                    {
                      "type": "url",
                      "offset": 12,
                      "length": 21
                    }
                ]                
            "#,
        )
        .unwrap();
        assert_eq!(
            parse_entities("我是中 #上班 不错  https://www.baidu.com 是我", &msgs)
                .iter()
                .filter_map(|e| e.message.get(e.range.clone()))
                .collect::<Vec<&str>>(),
            ["#上班", "https://www.baidu.com"]
        )
    }
}
