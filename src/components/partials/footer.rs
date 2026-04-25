use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_brands_icons::{FaGithub, FaLinkedin, FaLinkedinIn}};

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
                        Icon {
                            height: 16,
                            class: "inline-block mb-1",
                            icon: FaGithub
                        }
                        "Github"
                    }
                }
                li {
                    Link {
                        class: "text-ctp-blue border-b-2 hover:text-ctp-rosewater",
                        to: "https://www.linkedin.com/in/nick-hesemans/",
                        new_tab: true,
                        Icon {
                            height: 16,
                            class: "inline-block mb-1",
                            icon: FaLinkedin
                        }
                        "LinkedIn"
                    }
                }
            }
        }
    }
}