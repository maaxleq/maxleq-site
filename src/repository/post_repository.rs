use axum::BoxError;
use futures::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

use crate::{database::get_collection, entity::post::Post};

use super::pagination::Pagination;

pub async fn add_post(post: Post) -> Result<(), BoxError> {
    get_collection("posts")
        .await?
        .insert_one(post, None)
        .await?;
    Ok(())
}

pub async fn get_posts_newest_first(pagination: Pagination) -> Result<Vec<Post>, BoxError> {
    Ok(get_collection::<Post>("posts")
        .await?
        .find(
            None,
            FindOptions::builder()
                .sort(doc! {
                    "published_at": -1
                })
                .limit(pagination.limit.unwrap_or(usize::MAX) as i64)
                .skip(pagination.offset.unwrap_or(0) as u64)
                .build(),
        )
        .await?
        .try_collect()
        .await?)
}

pub async fn search_posts_by_title(
    title: String,
    pagination: Pagination,
) -> Result<Vec<Post>, BoxError> {
    Ok(get_collection::<Post>("posts")
        .await?
        .find(
            doc! {
                "title": {
                    "$regex": format!(".*{}.*", title),
                    "$options": "i"
                }
            },
            FindOptions::builder()
                .sort(doc! {
                    "published_at": -1
                })
                .limit(pagination.limit.unwrap_or(usize::MAX) as i64)
                .skip(pagination.offset.unwrap_or(0) as u64)
                .build(),
        )
        .await?
        .try_collect()
        .await?)
}
