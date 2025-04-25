use crate::{GameState, PointsData, Question, QuestionType};
use leptos::context::use_context;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

#[component]
pub fn Question(question: Question, index: usize) -> impl IntoView {
    let set_game_state = use_context::<ArcWriteSignal<GameState>>().expect("Error");
    let set_question_points = use_context::<ArcWriteSignal<PointsData>>().expect("Error");
    let read_question_points = use_context::<ArcReadSignal<PointsData>>().expect("Error");
    let (answered, set_answered) = signal(false);
    let multiplier_message = move || match question.multiplier {
        2. => "Double Jeopardy!!!",
        3. => "Triple Jeopardy!!!",
        4. => "Quadruple Jeopardy!!!",
        5. => "Quintuple Jeopardy!!!",
        _ => ""
    };

    set_question_points.update(|points_data: &mut PointsData| points_data.update_question_points(index, question.multiplier));

    view! {
        <div class="question-container">
            <button id="back-button" class="button-s" on:click=move |_| { set_game_state.set(GameState::Home) }>"Back"</button>
            <div id="value-notification">
                <h2>{multiplier_message}</h2>
                <h2>{move || read_question_points.get().current_question_points}" Points"</h2>
            </div>
            <div class="question">
                { move || match question.question.clone() {
                    QuestionType::Text(options) => view! {
                        <div id="text-question">
                            <h2>{options.text}</h2>
                        </div>
                    }.into_any(),
                    QuestionType::Audio(options) => view! {
                        <div id="audio-question">
                            <audio controls src={options.link}></audio>
                            <h2>{options.text}</h2>
                        </div>
                    }.into_any(),
                    QuestionType::Video(options) => view! {
                        <div id="video-question">
                            <iframe class=move || match options.blurred { true => "blurred", false => "" } src={options.link.clone()} title="YouTube video player" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
                            <h2>{options.text.clone()}</h2>
                        </div>
                    }.into_any(),
                    QuestionType::Image(options) => view! {
                        <div id="image-question">
                            <img class=move || match options.blurred { true => "blurred", false => "" } src={options.link} alt="Fard"/>
                            <h2>{options.text}</h2>
                        </div>
                    }.into_any(),
                }}
            </div>
            <div class="answer">
                { move || match answered.get() {
                    true => view! { <h2>{question.answer.clone()}</h2> }.into_any(),
                    false => view! { <button class="button-s" on:click=move |_| set_answered.update(|a| *a = !*a)>"Reveal Answer"</button> }.into_any(),
                }}
            </div>
        </div>
    }
}