use leptos::{component, view, IntoView};
use leptos::context::use_context;
use leptos::prelude::*;
use crate::{GameData, GameState};

#[component]
pub fn Tile(value: u32, location: (usize, usize), exhausted: bool) -> impl IntoView {
    let game_state_setter = use_context::<ArcWriteSignal<GameState>>().expect("Error");
    let game_data_setter = use_context::<ArcWriteSignal<GameData>>().expect("Error");

    view! {
        <button on:click=move |_| {
            game_state_setter.set(GameState::Question(location));
            game_data_setter.update(|data| data.categories[location.0].questions[location.1].answered = true);
        } class=move || match exhausted { true => "tile-exhausted", false => "tile" }>
            <h2>{value}</h2>
        </button>
    }
}