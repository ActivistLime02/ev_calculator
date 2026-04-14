use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Layout() -> Element {
    rsx! {
        document::Stylesheet {
            href: TAILWIND_CSS
        }

        nav {
            class: "bg-red-500",
            ol {
                class: "flex gap-4",
                li {
                    "Mama mia!"
                }
                li {
                    "Mama mia!"
                }
                li {
                    "Mama mia!"
                }
            }
        }
    }
}
