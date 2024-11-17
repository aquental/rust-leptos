use leptos::*;
use leptos_router::use_params;
use leptos_router::Params;
use serde::{Deserialize, Serialize};

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
    view! {
        {display_params}
    }
}
