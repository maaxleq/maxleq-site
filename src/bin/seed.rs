use dotenv::dotenv;
use maxleq_site::database::{get_collection};
use maxleq_site::entity::post::{Paragraph, TextSection};
use maxleq_site::{
    entity::{message::Message, post::Post},
};
use mongodb::bson::doc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use tower::BoxError;

async fn reset_posts() -> Result<(), BoxError> {
    get_collection::<Post>("posts").await?.delete_many(doc!{}, None).await?;
    Ok(())
}

async fn reset_messages() -> Result<(), BoxError> {
    get_collection::<Message>("messages").await?.delete_many(doc!{}, None).await?;
    Ok(())
}

fn get_posts() -> Result<Vec<Post>, BoxError> {
    Ok(vec![
        Post {
            title: String::from("How to Protect Against Attacks Using a Quantum Computer"),
            slug: String::from("protect-with-quantum-computer"),
            main_image_path: Some(String::from("https://hackernoon.com/_next/image?url=https%3A%2F%2Fcdn.hackernoon.com%2Fimages%2F-apa3or9.png&w=828&q=75")),
            published_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
            content: vec![
                Paragraph::Text(
                    vec![
                        TextSection::Plain(String::from("Quantum computers are much more powerful than devices we are used to and can perform well in various areas of life. At the same time, hacking with their help can instantly bring down digital systems in critical infrastructures. Even ")),
                        TextSection::Link {
                            destination: String::from("https://hackernoon.com/what-are-quantum-resistant-blockchains?ref=hackernoon.com"),
                            text: String::from("quantum-resistant blockchains")
                        },
                        TextSection::Plain(String::from(" have emerged."))
                    ]
                ),
                Paragraph::Heading {
                    level: maxleq_site::entity::post::HeadingLevel::H1,
                    text: String::from("Quantum Technologies")
                }
            ]
        }
    ])
}

fn get_messages() -> Result<Vec<Message>, BoxError> {
    Ok(vec![
        Message {
            sender_name: Some("John Doe".to_string()),
            sender_email_address: "john.doe@mail.com".to_string(),
            message: "Hello\nI am John".to_string(),
            subject: "Greetings".to_string(),
            sent_at: 0
        }
    ])
}

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    dotenv().ok();

    reset_posts().await?;
    reset_messages().await?;

    get_collection("posts").await?.insert_many(get_posts()?, None).await?;
    get_collection("messages").await?.insert_many(get_messages()?, None).await?;

    Ok(())
}
