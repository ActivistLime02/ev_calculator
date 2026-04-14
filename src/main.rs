use dioxus::prelude::*;
use crate::layout::Layout;

#[path ="components/layout.rs"]
mod layout;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Layout {}
    }
}
