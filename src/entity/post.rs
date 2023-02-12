use serde::{Serialize, Deserialize};

use super::timestamp::Timestamp;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum HeadingLevel {
    H1,
    H2,
    H3
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TextSection {
    Plain(String),
    Link {
        destination: String,
        text: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Paragraph {
    Text(Vec<TextSection>),
    Image {
        path: String,
        caption: Option<String>,
        alt: String
    },
    Quote(String),
    Heading {
        level: HeadingLevel,
        text: String
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub title: String,
    pub slug: String,
    pub main_image_path: Option<String>,
    pub content: Vec<Paragraph>,
    pub published_at: Timestamp
}
