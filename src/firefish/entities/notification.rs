use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::entities as MegalodonEntities;

use super::{reaction::map_reaction, Note, User};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    id: String,
    created_at: DateTime<Utc>,
    // is_read: bool,
    pub r#type: NotificationType,
    // user_id: Option<String>,
    user: Option<User>,
    note: Option<Note>,
    reaction: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    Follow,
    Mention,
    Reply,
    Renote,
    Quote,
    Reaction,
    PollVote,
    PollEnded,
    ReceiveFollowRequest,
    FollowRequestAccepted,
    GroupInvited,
    App,
    Unknown,
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationType::Follow => write!(f, "follow"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::Reply => write!(f, "reply"),
            NotificationType::Renote => write!(f, "renote"),
            NotificationType::Quote => write!(f, "quote"),
            NotificationType::Reaction => write!(f, "reaction"),
            NotificationType::PollVote => write!(f, "pollVote"),
            NotificationType::PollEnded => write!(f, "pollEnded"),
            NotificationType::ReceiveFollowRequest => write!(f, "receiveFollowRequest"),
            NotificationType::FollowRequestAccepted => write!(f, "followRequestAccepted"),
            NotificationType::GroupInvited => write!(f, "groupInvited"),
            NotificationType::App => write!(f, "app"),
            NotificationType::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<MegalodonEntities::notification::NotificationType> for NotificationType {
    fn from(value: MegalodonEntities::notification::NotificationType) -> Self {
        match value {
            MegalodonEntities::notification::NotificationType::Follow => NotificationType::Follow,
            MegalodonEntities::notification::NotificationType::Mention => NotificationType::Mention,
            MegalodonEntities::notification::NotificationType::Reblog => NotificationType::Renote,
            MegalodonEntities::notification::NotificationType::Reaction => {
                NotificationType::Reaction
            }
            MegalodonEntities::notification::NotificationType::PollVote => {
                NotificationType::PollVote
            }
            MegalodonEntities::notification::NotificationType::PollExpired => {
                NotificationType::PollEnded
            }
            MegalodonEntities::notification::NotificationType::FollowRequest => {
                NotificationType::ReceiveFollowRequest
            }
            MegalodonEntities::notification::NotificationType::GroupInvited => {
                NotificationType::GroupInvited
            }
            MegalodonEntities::notification::NotificationType::App => NotificationType::App,
            MegalodonEntities::notification::NotificationType::AdminReport => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::AdminSignup => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::Favourite => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::Move => NotificationType::Unknown,
            MegalodonEntities::notification::NotificationType::Status => NotificationType::Unknown,
            MegalodonEntities::notification::NotificationType::Unknown => NotificationType::Unknown,
            MegalodonEntities::notification::NotificationType::Update => NotificationType::Unknown,
        }
    }
}

impl From<NotificationType> for MegalodonEntities::notification::NotificationType {
    fn from(val: NotificationType) -> Self {
        match val {
            NotificationType::Follow => MegalodonEntities::notification::NotificationType::Follow,
            NotificationType::Mention => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Reply => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Renote => MegalodonEntities::notification::NotificationType::Reblog,
            NotificationType::Quote => MegalodonEntities::notification::NotificationType::Reblog,
            NotificationType::Reaction => {
                MegalodonEntities::notification::NotificationType::Reaction
            }
            NotificationType::PollVote => {
                MegalodonEntities::notification::NotificationType::PollVote
            }
            NotificationType::PollEnded => {
                MegalodonEntities::notification::NotificationType::PollExpired
            }
            NotificationType::ReceiveFollowRequest => {
                MegalodonEntities::notification::NotificationType::FollowRequest
            }
            NotificationType::FollowRequestAccepted => {
                MegalodonEntities::notification::NotificationType::Follow
            }
            NotificationType::GroupInvited => {
                MegalodonEntities::notification::NotificationType::GroupInvited
            }
            NotificationType::App => MegalodonEntities::notification::NotificationType::App,
            NotificationType::Unknown => MegalodonEntities::notification::NotificationType::Unknown,
        }
    }
}

impl From<Notification> for MegalodonEntities::Notification {
    fn from(val: Notification) -> Self {
        let emojis = if let Some(note) = &val.note {
            note.clone().emojis.unwrap_or_default()
        } else {
            [].to_vec()
        };
        let reactions = if let Some(reaction) = val.reaction {
            map_reaction(emojis, HashMap::<String, u32>::from([(reaction, 1)]), None)
        } else {
            [].to_vec()
        };
        let reaction = if reactions.len() > 0 {
            Some(reactions[0].clone())
        } else {
            None
        };
        MegalodonEntities::Notification {
            account: val.user.map(|u| u.into()),
            created_at: val.created_at,
            id: val.id,
            status: val.note.map(|n| n.into()),
            reaction,
            target: None,
            r#type: val.r#type.into(),
        }
    }
}
