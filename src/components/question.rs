use leptos::{component, view, IntoView};
use leptos::context::use_context;
use leptos::either::Either;
use leptos::ev::message;
use leptos::prelude::*;
use crate::{GameState, Question, QuestionType};
use crate::components::popup::Popup;

#[component]
pub fn Question(question: Question) -> impl IntoView {
    let setter = use_context::<ArcWriteSignal<GameState>>().expect("Error");
    let (answered, set_answered) = signal(false);

    let multipler = question.multiplier;

    view! {
        { move || if multipler > 1. {
            match multipler {
                2. => view! { <Popup message="Double Jeopardy!!!".to_string() />}.into_any(),
                3. => view! { <Popup message="Triple Jeopardy!!!".to_string() />}.into_any(),
                4. => view! { <Popup message="Quadruple Jeopardy!!!".to_string() />}.into_any(),
                5. => view! { <Popup message="Quintuple Jeopardy!!!".to_string() />}.into_any(),
                _ => view! { <Popup message={question.multiplier.clone().to_string()} />}.into_any()
            }
        } else {
            { view! { } }.into_any()
        }}
        <div class="question-container">
            <button id="back-button" class="button-s" on:click=move |_| { setter.set(GameState::Home) }>"Back"</button>
            <div class="question">
                { move || match question.question.clone() {
                    QuestionType::Text(text) => view! {
                        <h2 id="question-text">{text.text}</h2>
                    }.into_any(),
                    QuestionType::Audio(options) => view! {
                        <audio controls id="question-audio" src={options.link}></audio>
                        <h2 id="question-text">{options.text}</h2>
                    }.into_any(),
                    QuestionType::Video(options) => view! {
                        <iframe id="question-video" class=move || match options.blurred { true => "blurred", false => "" } src={options.link.clone()} title="YouTube video player" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                        <h2 id="question-text">{options.text.clone()}</h2>
                    }.into_any(),
                    QuestionType::Image(options) => view! {
                        <img id="question-image" src={options.link} alt="Fard"/>
                        <h2 id="question-text">{options.text}</h2>
                    }.into_any(),
                }}
            </div>
            <div class="answer">
                { move || match answered.get() {
                    true => view! { <h2>{question.answer.clone()}</h2> }.into_any(),
                    false => view! { }.into_any(),
                }}
                <button class="button-s" on:click=move |_| set_answered.update(|a| *a = !*a)>"Reveal Answer"</button>
            </div>
        </div>
    }
}