use serde::{Serialize, Deserialize};

use super::timestamp::Timestamp;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub sender_name: Option<String>,
    pub sender_email_address: String,
    pub subject: String,
    pub message: String,
    pub sent_at: Timestamp
}
