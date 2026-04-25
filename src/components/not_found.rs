use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let full_url = "/".to_string() + &segments.join("/");
    rsx! {
        h1 {
            class: "text-ctp-rosewater text-3xl mt-4 mb-2",
            "404 page not found"
        }
        p {
            class: "",
            "Path "
            span {
                class: "text-ctp-blue",
                "{full_url}"
            }
            " does not exist."
        }
        Link {
            class: "
                bg-ctp-lavender hover:bg-ctp-rosewater-800 text-ctp-base
                px-4 py-2 my-2
                rounded-md
                cursor-pointer
                inline-block
            ",
                to: Route::Calculator {},
                "Return to calculator"
        }
    }
}