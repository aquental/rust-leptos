use leptos::*;
use leptos_meta::*;
use leptos_router::*;
#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav>"esta é a navbar"</nav>
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
                //<Routes>
                    //<Route path="" view=|| view! { <h1>"Bem vindo ao Leptos."</h1> }/>
                    //<Route path="/edit/:post_id?" view=ViewPost/>
                    //<Route path="/view/:post_id?" view=Editost/>
                //</Routes>
            </main>
        </Router>

    }
}

#[component]
fn BlogDescription() -> impl IntoView {
    view! {
        <h1>"BlogDescription"</h1>
    }
}

#[component]
fn BlogPreviews() -> impl IntoView {
    view! {
        <h1>"BlogPreviews"</h1>
    }
}
