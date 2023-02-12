use lazy_static::lazy_static;
use mongodb::{Client, Collection, Database};
use std::{env, sync::Arc};
use tower::BoxError;

use crate::error::program_error::COULD_NOT_INIT_DB_CLIENT;

lazy_static! {
    static ref DB_CLIENT: Arc<Client> = Arc::new(futures::executor::block_on(async {
        init_client().await.expect(COULD_NOT_INIT_DB_CLIENT)
    }));
}

pub async fn init_client() -> Result<Client, BoxError> {
    let uri_string = env::var("MONGO_URI_STRING")?;

    let client = Client::with_uri_str(uri_string.as_str()).await?;

    Ok(client)
}

pub async fn get_client() -> Arc<Client> {
    DB_CLIENT.clone()
}

pub async fn get_database() -> Result<Database, BoxError> {
    let db_name = env::var("MONGO_DATABASE")?;
    Ok(get_client().await.database(db_name.as_str()))
}

pub async fn get_collection<T>(name: &str) -> Result<Collection<T>, BoxError> {
    Ok(get_database().await?.collection(name))
}
