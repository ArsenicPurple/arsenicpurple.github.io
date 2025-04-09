use leptos::{component, view, IntoView};
use leptos::context::use_context;
use leptos::prelude::*;
use crate::{GameState, Question, QuestionType};

#[component]
pub fn Question(question: Question) -> impl IntoView {
    let setter = use_context::<ArcWriteSignal<GameState>>().expect("Error");
    let (answered, set_answered) = signal(false);

    view! {
        <div id="back-button-container">
            <button class="button-s" on:click=move |_| { setter.set(GameState::Home) }>"Back"</button>
        </div>
        <div class="question">
            { move || match question.question.clone() {
                QuestionType::Text(text) => view! {
                    <h2 id="question-text">{text}</h2>
                }.into_any(),
                QuestionType::Audio(link, text) => view! {
                    <audio controls id="question-audio" src={link}></audio>
                    <h2 id="question-text">{text}</h2>
                }.into_any(),
                QuestionType::Video(link, text) => view! {
                    <iframe id="question-video" width="560" height="315" src={link} title="YouTube video player" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                    <h2 id="question-text">{text}</h2>
                }.into_any(),
                QuestionType::Image(link, text) => view! {
                    <img id="question-image" src={link} alt="Italian Trulli"/>
                    <h2 id="question-text">{text}</h2>
                }.into_any(),
            }}
            { move || match answered.get() {
                true => view! { <h2>{question.answer.clone()}</h2> }.into_any(),
                false => view! { }.into_any(),
            }}
            <button class="button-s" on:click=move |_| set_answered.update(|a| *a = !*a)>"Reveal Answer"</button>
        </div>
    }
}