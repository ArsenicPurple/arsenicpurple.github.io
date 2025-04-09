use leptos::{component, view, IntoView};
use leptos::context::{provide_context, use_context};
use leptos::prelude::*;
use crate::{GameData, GameState};
use crate::components::column::Column;
use crate::components::question::Question;

#[component]
pub fn Game() -> impl IntoView {
    let (read_game_state, set_game_state) = arc_signal(GameState::Home);
    provide_context(set_game_state);
    provide_context(read_game_state);

    let read_game_data = use_context::<ArcReadSignal<GameData>>().expect("Error");
    let read_game_state = use_context::<ArcReadSignal<GameState>>().expect("Error");

    view! {
        { move || match read_game_state.get() {
            GameState::Home => view! {
                {
                    read_game_data.get().categories.iter().enumerate()
                        .map(|(id, category)| view! { <Column id=id title=category.title.clone() questions=category.questions.clone() /> })
                        .collect::<Vec<_>>()
                }
            }.into_any(),
            GameState::Question((column, row)) => view! {
                <Question question=read_game_data.get().categories[column].questions[row].clone()/>
            }.into_any()
        }}
    }
}