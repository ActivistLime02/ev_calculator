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
    // Layout applies to all the routes below, apparently.
    #[layout(DefaultLayout)]
    #[route("/")]
    Calculator,

    // Catch-all route. Returns 404 when the user accesses a page that does not exist.
    // Must be at the end, this enum works like a firewall rule.
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
