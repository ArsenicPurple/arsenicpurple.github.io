use leptos::{component, view, IntoView};
use leptos::context::{provide_context, use_context};
use leptos::prelude::*;
use crate::{GameData, GameState};
use crate::components::column::Column;
use crate::components::question::Question;

#[component]
pub fn Game() -> impl IntoView {
    let (game_state, set_game_state) = arc_signal(GameState::Home);
    provide_context(set_game_state);

    let game_data_reader = use_context::<ArcReadSignal<GameData>>().expect("Error");

    view! {
        <main>
            { move || match game_state.get() {
                GameState::Home => view! {
                    {
                        game_data_reader.get().categories.iter().enumerate()
                            .map(|(id, category)| view! { <Column id=id title=category.title.clone() questions=category.questions.clone() /> })
                            .collect::<Vec<_>>()
                    }
                }.into_any(),
                GameState::Question((column, row)) => view! {
                    <Question question=game_data_reader.get().categories[column].questions[row].clone()/>
                }.into_any()
            }}
        </main>
    }
}