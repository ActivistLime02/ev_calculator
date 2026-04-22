use dioxus::prelude::*;

use crate::Route;

use crate::Header;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn DefaultLayout() -> Element {
    rsx! {
        document::Stylesheet {
            href: TAILWIND_CSS
        }

        div {
            class: "bg-ctp-base text-ctp-text latte",
            Header {}

            // Container
            div {
                class: "container my-4 mx-2",
                Outlet::<Route> {}
            }
        }
    }
}
