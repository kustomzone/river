use dioxus::prelude::*;
use ed25519_dalek::VerifyingKey;
use common::ChatRoomStateV1;

#[component]
pub fn ChatRoomModal(
    current_room: Signal<Option<VerifyingKey>>,
    current_room_state: Memo<Option<ChatRoomStateV1>>
) -> Element {
    let mut show = use_signal(|| false);
    let modal_type = use_signal(String::new);
    let modal_name = use_signal(String::new);

    let close_modal = move |_| show.set(false);

    rsx! {
        div { 
            class: format_args!("modal {}", if *show.read() { "is-active" } else { "" }),
            div { class: "modal-background", onclick: close_modal }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    p { class: "modal-card-title", "{modal_type.read()}: {modal_name.read()}" }
                    button {
                        class: "delete",
                        "aria-label": "close",
                        onclick: close_modal
                    }
                }
                section { class: "modal-card-body",
                    // TODO: Implement dynamic content based on modal type and current_room_state
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