use leptos::{component, view, IntoView};
use leptos::prelude::*;
use crate::components::tile::Tile;
use crate::Question;

#[component]
pub fn Column(id: usize, title: String, questions: Vec<Question>, answered: Vec<bool>) -> impl IntoView {
    view! {
        <div class="column">
            <h2>{title}</h2>
            {
                questions.into_iter().enumerate()
                    .map(|(index, _)| view! { <Tile location=(id, index) answered=answered[index] /> })
                    .collect::<Vec<_>>()
            }
        </div>
    }
}