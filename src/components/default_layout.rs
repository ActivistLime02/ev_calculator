use dioxus::prelude::*;

use crate::Route;

use crate::Header;
use crate::Footer;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[component]
pub fn DefaultLayout() -> Element {
    rsx! {
        document::Stylesheet {
            href: TAILWIND_CSS
        }

        // Acording to a github issue, this should behave like react 19.
        // Hoist to head
        document::Link {
            rel: "stylesheet",
            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/7.0.1/css/all.min.css",
            integrity: "sha512-2SwdPD6INVrV/lHTZbO2nodKhrnDdJK9/kg2XD1r9uGqPo1cUbujc+IYdlYdEErWNu69gVcYgdxlmVmzTWnetw==",
            crossorigin: "anonymous",
            referrerpolicy: "no-referrer"
        }

        document::Title {
            "EV Charge Time Calculator"
        }

        document::Link {
            rel: "icon",
            href: FAVICON
        }

        div {
            class: "
                bg-ctp-base text-ctp-text latte
                flex flex-col min-h-screen
            ",
            Header {}

            // Container
            div {
                class: "
                    container my-4 mx-auto p-4
                    grow
                ",
                Outlet::<Route> {}
            }

            Footer {}
        }
    }
}
