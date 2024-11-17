use leptos::*;

use crate::model::blog_post::Post;

#[component]
pub fn BlogPost(post: Post) -> impl IntoView {
    view! {
        <div>
            <h1>{post.title}</h1>
            <p>{post.text}</p>
        </div>
    }
}
