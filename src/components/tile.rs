use crate::{GameState, PointsData};
use leptos::context::use_context;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Tile(location: (usize, usize), answered: bool) -> impl IntoView {
    let set_game_state = use_context::<ArcWriteSignal<GameState>>().expect("Error");
    let set_answered = use_context::<WriteSignal<Vec<Vec<bool>>>>().expect("Error");
    let read_points_data = use_context::<ArcReadSignal<PointsData>>().expect("Error");

    view! {
        <button on:click=move |_| {
            set_game_state.set(GameState::Question(location));
            set_answered.update(|data| data[location.0][location.1] = true);
        } class=move || match answered { true => "tile-answered", false => "tile" }>
            <h2>{move || calculate_display_points(location.1, read_points_data.get().board_multiplier)}</h2>
        </button>
    }
}

pub fn calculate_display_points(index: usize, board_multiplier: f32) -> u32 {
    (((index + 1) * 100) as f32 * board_multiplier) as u32
}