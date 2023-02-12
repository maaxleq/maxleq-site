use axum::BoxError;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

use crate::{database::get_collection, entity::message::Message};

pub async fn add_message(message: Message) -> Result<(), BoxError> {
    get_collection("messages")
        .await?
        .insert_one(message, None)
        .await?;
    Ok(())
}

pub async fn get_messages_newest_first() -> Result<Vec<Message>, BoxError> {
    Ok(get_collection::<Message>("messages")
        .await?
        .find(
            None,
            FindOptions::builder()
                .sort(doc! {
                    "sent_at": -1
                })
                .build(),
        )
        .await?
        .try_collect()
        .await?)
}
