use crate::GameState;
use crate::GameState::Question;
use leptos::context::use_context;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use std::collections::HashMap;

#[component]
pub fn Score(player: String, score: u32) -> impl IntoView {
    let set_scores = use_context::<ArcWriteSignal<HashMap<String, u32>>>().expect("Error");
    let read_game_state = use_context::<ArcReadSignal<GameState>>().expect("Error");

    let key = player.clone();
    let set_scores_copy = set_scores.clone();
    let read_game_state_copy = read_game_state.clone();
    let add_score = move || {
        let Question((_, row)) = read_game_state_copy.get() else { return };
        set_scores_copy.update(|map| { map.entry(key.clone()).and_modify(|score| { *score = score.saturating_add((100 * (row as u32 + 1))); }); });
    };

    let key = player.clone();
    let set_scores_copy = set_scores.clone();
    let read_game_state_copy = read_game_state.clone();
    let sub_score = move || {
        let Question((_, row)) = read_game_state_copy.get() else { return };
        set_scores_copy.update(|map| { map.entry(key.clone()).and_modify(|score| { *score = score.saturating_sub((100 * (row as u32 + 1))); }); });
    };

    view! {
        <div class="scorebox">
            <h3>{player.clone()}</h3>
            <input type="number" value={score} />
            <div class="scorebox-buttons">
                <button on:click=move |_| { add_score() } class="scorebox-button">+</button>
                <button on:click=move |_| { sub_score() } class="scorebox-button">-</button>
            </div>
        </div>
    }
}