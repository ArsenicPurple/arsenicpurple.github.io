use crate::GameState;
use leptos::context::use_context;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Tile(value: u32, location: (usize, usize), answered: bool) -> impl IntoView {
    let set_game_state = use_context::<ArcWriteSignal<GameState>>().expect("Error");
    let set_answered = use_context::<WriteSignal<Vec<Vec<bool>>>>().expect("Error");

    view! {
        <button on:click=move |_| {
            set_game_state.set(GameState::Question(location));
            set_answered.update(|data| data[location.0][location.1] = true);
        } class=move || match answered { true => "tile-answered", false => "tile" }>
            <h2>{value}</h2>
        </button>
    }
}