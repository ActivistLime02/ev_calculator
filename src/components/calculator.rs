use dioxus::prelude::*;

use crate::input::Input;
use crate::label::Label;

#[component]
pub fn Calculator() -> Element {
    let mut battery_capacity = use_signal(|| 0.0);
    let mut charging_speed = use_signal(|| 0.0);

    let mut result = use_signal(|| 0.0);

    rsx! {
        div {
            class: "mx-2",
            div {
                class: "my-2",
                Label {
                    html_for: "battery_capacity",
                    "Battery capacity (kWh)"
                }
                Input {
                    r#type: "number",
                    id: "battery_capacity",
                    class: "border-2 mt-1 p-2 rounded-md",
                    value: "{battery_capacity}",
                    oninput: move |evt: FormEvent| battery_capacity.set(evt.value().parse().unwrap_or(0.0)),
                }
            }

            div {
                class: "my-2",
                Label {
                    html_for: "charging_speed",
                    "Charging speed (kW)"
                }
                Input {
                    r#type: "number",
                    id: "charging_speed",
                    class: "border-2 mt-1 p-2 rounded-md",
                    value: "{charging_speed}",
                    oninput: move |evt: FormEvent| charging_speed.set(evt.value().parse().unwrap_or(0.0)),
                }
            }

            if result() != 0.0 {
                div {
                    class: "
                        rounded-md bg-ctp-surface0 p-2
                        relative
                    ",

                    h2 {
                        "Result:"
                    }

                    i {
                        class: "
                            fa-solid fa-x
                            absolute
                            top-2 right-1
                            cursor-pointer
                        ",
                        onclick: move |_| result.set(0.0)
                    }

                    p { "You need to charge for {(result() as f64).trunc()} hours and {((result() as f64).fract() * 60.0).ceil()} minutes." }
                }
            }

            button {
                class: "
                    bg-ctp-rosewater hover:bg-ctp-rosewater-800 text-ctp-base
                    px-4 py-2 my-2
                    rounded-md
                    cursor-pointer
                ",
                onclick: move |_| {
                    result.set(battery_capacity() / charging_speed());
                },
                "Calculate"
            }
        }
    }
}
