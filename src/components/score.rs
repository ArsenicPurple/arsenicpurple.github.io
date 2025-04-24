use crate::PointsData;
use leptos::context::use_context;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use std::collections::HashMap;

#[component]
pub fn Score(player: String, score: u32) -> impl IntoView {
    let set_scores = use_context::<ArcWriteSignal<HashMap<String, u32>>>().expect("Error");
    let read_question_points = use_context::<ArcReadSignal<PointsData>>().expect("Error");

    let key = player.clone();
    let set_scores_copy = set_scores.clone();
    let read_question_points_copy = read_question_points.clone();
    let add_score = move || { set_scores_copy.update(|map| {
        map.entry(key.clone()).and_modify(|score| { *score = score.saturating_add(read_question_points_copy.get().current_question_points); }); });
    };

    let key = player.clone();
    let set_scores_copy = set_scores.clone();
    let read_question_points_copy = read_question_points.clone();
    let sub_score = move || { set_scores_copy.update(|map| {
        map.entry(key.clone()).and_modify(|score| { *score = score.saturating_sub(read_question_points_copy.get().current_question_points); }); });
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