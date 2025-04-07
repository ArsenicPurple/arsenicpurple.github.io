use std::collections::HashMap;
use leptos::{component, view, IntoView};
use leptos::context::use_context;
use leptos::prelude::*;

#[component]
pub fn Score(player: String, score: u32) -> impl IntoView {
    let set_scores = use_context::<ArcWriteSignal<HashMap<String, u32>>>().expect("Error");
    view! {
        <div class="scorebox">
            <h3>{player.clone()}</h3>
            <input type="number" placeholder={score} />
        </div>
    }
}