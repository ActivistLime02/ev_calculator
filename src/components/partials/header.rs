use dioxus::prelude::*;
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
            NavbarItem {
                index: 1usize,
                value: "portfolio".to_string(),
                to: "https://nickhesemans.be",
                "Portfolio"
            }
        }
    }
}
