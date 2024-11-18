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
        <div class="dark:bg-gray-800 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                //title
                <a href="/" class="text-2xl font-bold">Moonbound</a>
                // nav bar
                <nav>
                    <ul class="flex space-x-4">
                        <li><a href="/" class="hover:text-blue-400">"Blog"</a></li>
                        <li><a href="/edit" class="hover:text-blue-400">"Create"</a></li>
                    </ul>
                </nav>
            </div>
        </div>
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
            <main class="bg-gray-700 text-gray-200 p-8 h-full">
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
