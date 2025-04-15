use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Popup(message: String) -> impl IntoView {
    let (read_visible, set_visible) = signal(true);
    move || match read_visible.get() {
        true => view! {
            <button
                on:click=move |_| set_visible.set(false)
                class="popup"
            >
                {message.clone()}
            </button>
        }.into_any(),
        false => view! { }.into_any()
    }
}