use chrono::Local;
use leptos::{server, ServerFnError};

use crate::model::blog_post::Post;

#[server(UpsertPost, "/api")]
pub async fn upsert_post(
    id: Option<String>,
    dt: String,
    image_url: String,
    title: String,
    text: String,
) -> Result<String, ServerFnError> {
    Ok(String::from("ok"))
}

#[server(GetPost, "/api")]
pub async fn get_post(id: String) -> Result<Post, ServerFnError> {
    Ok(Post {
        id: "1".to_string(),
        dt: Local::now().naive_local(),
        image_url: "".to_string(),
        title: "https://bit.ly/3t0bA61".to_string(),
        text: "I spent some time at the beach today. It was amazing!".to_string(),
    })
}

#[server(GetPreviews, "/api")]
pub async fn get_previews() -> Result<Vec<Post>, ServerFnError> {
    Ok(vec![])
}
