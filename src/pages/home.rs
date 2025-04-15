use crate::components::controls::Controls;
use crate::components::game::Game;
use crate::components::scoreboard::Scoreboard;
use crate::{create_answered_storage, GameData};
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Home() -> impl IntoView {
    let (read_answered, set_answered) = signal(create_answered_storage(5, 5));
    let (read_game_data, set_game_data) = arc_signal(GameData::default());
    let (read_scores, set_scores) = arc_signal(HashMap::<String, u32>::new());
    provide_context(set_scores);
    provide_context(read_scores);
    provide_context(set_answered);
    provide_context(read_answered);
    provide_context(set_game_data);
    provide_context(read_game_data);

    view! {
        <nav>
            <Controls/>
        </nav>
        <hr/>
        <main>
            <Game/>
        </main>
        <hr/>
        <footer>
            <Scoreboard/>
        </footer>
    }
}


