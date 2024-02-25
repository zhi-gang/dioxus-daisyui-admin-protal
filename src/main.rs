#![allow(non_snake_case)]

use dioxus_daisyui_admin_protal::app::App;
use log::{info, warn, LevelFilter};

fn main() {

    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    // launch the web app
    #[cfg(feature = "web")]
    dioxus_web::launch(App);

    #[cfg(feature = "desktop")]
    dioxus_desktop::launch(App);

    #[cfg(feature = "default")]
    warn!("please set the features in 'web' and 'desktop'.");

    info!("starting...")
}

/* 
use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch(App);

    #[cfg(not(feature = "web"))]
    dioxus_desktop::launch(App);
}

// Turn off rustfmt since we're doing layouts and routes in the same enum
#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
enum Route {
    // Wrap Home in a Navbar Layout
    #[layout(NavBar)]
        // The default route is always "/" unless otherwise specified
        #[route("/")]
        Home {},

        // Wrap the next routes in a layout and a nest
        #[nest("/blog")]
        #[layout(Blog)]
            // At "/blog", we want to show a list of blog posts
            #[route("/")]
            BlogList {},

            // At "/blog/:name", we want to show a specific blog post, using the name slug
            #[route("/:name")]
            BlogPost { name: String },

        // We need to end the blog layout and nest
        // Note we don't need either - we could've just done `/blog/` and `/blog/:name` without nesting,
        // but it's a bit cleaner this way
        #[end_layout]
        #[end_nest]

    // And the regular page layout
    #[end_layout]

    // Add some redirects for the `/myblog` route
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogList {})]
        #[redirect("/:name", |name: String| Route::BlogPost { name })]
    #[end_nest]

    // Finally, we need to handle the 404 page
    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

#[component]
fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::BlogList {}, "Blog" }
        }
        Outlet::<Route> {}
    })
}

#[component]
fn Home(cx: Scope) -> Element {
    cx.render(rsx! { h1 { "Welcome to the Dioxus Blog!" } })
}

#[component]
fn Blog(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
    })
}

#[component]
fn BlogList(cx: Scope) -> Element {
    cx.render(rsx! {
        h2 { "Choose a post" }
        div { id: "blog-list",
            Link { to: Route::BlogPost { name: "Blog post 1".into() },
                "Read the first blog post"
            }
            Link { to: Route::BlogPost { name: "Blog post 2".into() },
                "Read the second blog post"
            }
        }
    })
}

// We can use the `name` slug to show a specific blog post
// In theory we could read from the filesystem or a database here
#[component]
fn BlogPost(cx: Scope,name: String) -> Element {
    let contents = match name.as_str() {
        "Blog post 1" => "This is the first blog post. It's not very interesting.",
        "Blog post 2" => "This is the second blog post. It's not very interesting either.",
        _ => "This blog post doesn't exist.",
    };

    cx.render(rsx! {
        h2 { "{name}" }
        p { "{contents}" }
    })
}

#[component]
fn PageNotFound(cx: Scope,route: Vec<String>) -> Element {
    cx.render(rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    })
}

#[component]
fn App(cx: Scope) -> Element {
    render! {
        style { include_str!("../public/router.css") }
        div{
            Router::<Route> {}
        }
    }
} */