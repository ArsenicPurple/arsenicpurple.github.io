use crate::components::controls::Controls;
use crate::components::game::Game;
use crate::components::scoreboard::Scoreboard;
use crate::GameData;
use leptos::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Home() -> impl IntoView {
    let scores: HashMap<String, u32> = HashMap::new();
    let (read_game_data, set_game_data) = arc_signal(GameData::default());
    let (read_scores, set_scores) = arc_signal(scores);
    provide_context(set_scores);
    provide_context(read_scores);
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


