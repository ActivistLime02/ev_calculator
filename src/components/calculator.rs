use dioxus::prelude::*;

use crate::input::Input;
use crate::label::Label;

#[component]
pub fn Calculator() -> Element {
    let mut battery_capacity = use_signal(|| 0.0);
    let mut charging_speed = use_signal(|| 0.0);
    let mut charge_from_percentage = use_signal(|| 0);
    let mut charge_to_percentage = use_signal(|| 100);

    let mut result = use_signal(|| 0.0);

    fn calculate_charging_time(
        battery_capacity: Signal<f64>,
        charging_speed: Signal<f64>,
        charge_from_percentage: Signal<i32>,
        charge_to_percentage: Signal<i32>,
        mut result: Signal<f64>,
    ) {
        let percentage_to_charge: f64 = (charge_to_percentage() as f64 - charge_from_percentage() as f64) / 100.0;
        if charging_speed() != 0.0 {
            result.set(battery_capacity() * percentage_to_charge / charging_speed());
        } else {
            result.set(0.0);
        }
    }

    rsx! {
        div {
            class: "@container",
            div {
                class: "
                    grid gap-4 grid-cols-1
                    @lg:grid-cols-2
                    my-2
                ",
                div {
                    Label {
                        html_for: "battery_capacity",
                        "Battery capacity (kWh)"
                    }
                    Input {
                        r#type: "number",
                        id: "battery_capacity",
                        class: "border-2 mt-1 p-2 rounded-md w-full",
                        value: "{battery_capacity}",
                        oninput: move |evt: FormEvent| battery_capacity.set(evt.value().parse().unwrap_or(0.0)),
                    }
                }

                div {
                    Label {
                        html_for: "charging_speed",
                        "Charging speed (kW)"
                    }
                    Input {
                        r#type: "number",
                        id: "charging_speed",
                        class: "border-2 mt-1 p-2 rounded-md w-full",
                        value: "{charging_speed}",
                        oninput: move |evt: FormEvent| charging_speed.set(evt.value().parse().unwrap_or(0.0)),
                    }
                }

                div {
                    Label {
                        html_for: "start_soc",
                        "Current SoC (%)"
                    }
                    Input {
                        r#type: "number",
                        id: "start_soc",
                        class: "border-2 mt-1 p-2 rounded-md w-full",
                        value: "{charge_from_percentage}",
                        oninput: move |evt: FormEvent| charge_from_percentage.set(evt.value().parse().unwrap_or(0)),
                    }
                }

                div {
                    Label {
                        html_for: "stop_soc",
                        "Desired SoC (%)"
                    }
                    Input {
                        r#type: "number",
                        id: "stop_soc",
                        class: "border-2 mt-1 p-2 rounded-md w-full",
                        value: "{charge_to_percentage}",
                        oninput: move |evt: FormEvent| charge_to_percentage.set(evt.value().parse().unwrap_or(0)),
                    }
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
                    calculate_charging_time(
                        battery_capacity,
                        charging_speed,
                        charge_from_percentage,
                        charge_to_percentage,
                        result
                    );
                },
                "Calculate"
            }
        }
    }
}
