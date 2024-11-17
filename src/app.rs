use leptos::*;
use leptos_meta::*;
use leptos_router::*;

//use crate::component::blog_post::BlogPost;
use crate::component::blog_preview::BlogPreviews;
use crate::component::edit_post::EditPost;
use crate::component::view_post::ViewPost;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav>"<navbar>"</nav>
    }
}
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>"[footer]"</footer>
    }
}
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/moonbound.css"/>

        // sets the document title
        <Title text="AQuental"/>

        <Navbar/>
        <Router>
            <main>
                <Routes>
                    <Route path="" view=BlogPreviews/>
                    <Route path="/edit/:post_id?" view=ViewPost/>
                    <Route path="/view/:post_id?" view=EditPost/>
                </Routes>
            </main>
        </Router>
        <Footer/>
    }
}
