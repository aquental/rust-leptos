use leptos::*;
use leptos_router::use_params;
use leptos_router::Params;
use serde::{Deserialize, Serialize};

use crate::model::blog_post::Post;

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params = use_params::<EditPostParams>();
    let display_params = move || match params.get() {
        Ok(EditPostParams { post_id: Some(s) }) => s,
        _ => "".to_string(),
    };
    let (post, set_post) = create_signal(Post::new_empty());
    view! {
        <div class="flex h-screen">
            <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
                <form>

                </form>
            </div>
        </div>
    }
}
