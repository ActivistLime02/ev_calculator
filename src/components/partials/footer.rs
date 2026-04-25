use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "bg-ctp-surface0
                p-4
            ",
            ul {
                class: "",
                li {
                    Link {
                        class: "text-ctp-blue border-b-2 hover:text-ctp-rosewater",
                        to: "https://github.com/ActivistLime02",
                        new_tab: true,
                        i { class: "fa-brands fa-github" } " Github"
                    }
                }
                li {
                    Link {
                        class: "text-ctp-blue border-b-2 hover:text-ctp-rosewater",
                        to: "https://www.linkedin.com/in/nick-hesemans/",
                        new_tab: true,
                        i { class: "fa-brands fa-linkedin" } " LinkedIn"
                    }
                }
            }
        }
    }
}