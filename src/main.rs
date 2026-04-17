use dioxus::prelude::*;

mod components;

use crate::components::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(DefaultLayout)]
    #[route("/")]
    Calculator,

    // Catch-all route. Returns 404 when the user accesses a page that does not exist.
    // #[route("/:..segments")]
    // PageNotFound { segments: Vec<String> },
}
