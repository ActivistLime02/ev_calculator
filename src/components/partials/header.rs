use dioxus::prelude::*;
// use dioxus_primitives::navbar::{Navbar, NavbarItem};
use crate::navbar::*;
use crate::Route;

#[component]
pub fn Header() -> Element {
    rsx! {
        Navbar {
            NavbarItem {
                index: 0usize,
                value: "calculator".to_string(),
                to: Route::Calculator,
                "Calculator"
            }
        }
    }
}
