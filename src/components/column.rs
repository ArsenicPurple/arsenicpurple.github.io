use leptos::{component, view, IntoView};
use leptos::prelude::*;
use crate::components::tile::Tile;
use crate::Question;

#[component]
pub fn Column(id: usize, title: String, questions: Vec<Question>) -> impl IntoView {
    view! {
        <div class="column">
            <h2>{title}</h2>
            {
                questions.into_iter().enumerate()
                    .map(|(index, question)| view! { <Tile value=((index+1)*100) as u32 location=(id, index) exhausted=question.answered /> })
                    .collect::<Vec<_>>()
            }
        </div>
    }
}