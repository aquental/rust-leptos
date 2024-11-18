use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn new_empty() -> Post {
        Post {
            id: "".to_string(),
            dt: Local::now().naive_utc(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}
