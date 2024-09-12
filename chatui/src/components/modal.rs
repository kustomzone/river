use dioxus::prelude::*;

#[component]
pub fn Modal(
    show: Signal<bool>,
    modal_type: Signal<String>,
    modal_name: Signal<String>,
) -> Element {
    let close_modal = move |_| show.set(false);

    rsx! {
        div { class: "modal {if *show.get() { "is-active" } else { "" }}",
            div { class: "modal-background", onclick: close_modal }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "{modal_type.get()}: {modal_name.get()}" }
                    button {
                        class: "delete",
                        "aria-label": "close",
                        onclick: close_modal
                    }
                }
                section { class: "modal-card-body",
                    // TODO: Implement dynamic content based on modal type
                    "Modal content goes here"
                }
                footer { class: "modal-card-foot",
                    button {
                        class: "button is-success",
                        onclick: move |_| {
                            // TODO: Implement save changes logic
                            show.set(false);
                        },
                        "Save changes"
                    }
                    button {
                        class: "button",
                        onclick: close_modal,
                        "Cancel"
                    }
                }
            }
        }
    }
}