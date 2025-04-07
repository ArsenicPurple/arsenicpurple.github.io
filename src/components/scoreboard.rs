use std::collections::HashMap;
use leptos::{component, view, IntoView};
use leptos::context::use_context;
use leptos::prelude::*;
use crate::components::score::Score;

#[component]
pub fn Scoreboard() -> impl IntoView {
    let set_scores = use_context::<ArcWriteSignal<HashMap<String, u32>>>().expect("Error");
    let read_scores = use_context::<ArcReadSignal<HashMap<String, u32>>>().expect("Error");
    let (read_name, set_name) = signal("".to_string());
    view! {
        <div id="scoreboard">
            <div id="scoreboard-controls">
                <div id="scoreboard-addplayer">
                    <input on:input:target=move |ev| {
                        set_name.set(ev.target().value());
                    } type="text" placeholder="Player" prop:value=read_name />
                    <button on:click=move |_| { set_scores.update(|scores| { scores.insert(read_name.get(), 0); }); } class="button-s">"Add Player"</button>
                </div>
            </div>
            { move || read_scores.get().iter()
                .map(|(key, value)| { view! { <Score player=key.clone() score=value.clone() /> }})
                .collect::<Vec<_>>()
            }
        </div>
    }
}