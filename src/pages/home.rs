use std::collections::HashMap;
use leptos::prelude::*;
use crate::components::game::Game;
use crate::components::scoreboard::Scoreboard;
use crate::GameData;

#[component]
pub fn Home() -> impl IntoView {
    let once = OnceResource::<GameData>::new(async { GameData::default() });
    let scores: HashMap<String, u32> = HashMap::new();
    let (read_scores, set_scores) = arc_signal(scores);
    provide_context(read_scores);
    provide_context(set_scores);

    view! {
        <Suspense
            fallback=move || view! { <h1>"Loading..."</h1> }
        >
        { move || match once.get() {
            None => view! { <h1>"Loading Game"</h1> }.into_any(),
            Some(game_data) => {
                let (read_game_data, set_game_data) = arc_signal(game_data.clone());
                provide_context(set_game_data);
                provide_context(read_game_data);

                view! {
                    <nav>
                        <h1>{game_data.title.clone()}</h1>
                    </nav>
                    <hr/>
                    <Game/>
                    <hr/>
                    <footer>
                        <Scoreboard/>
                    </footer>
                }
            }.into_any(),
        }}
        </Suspense>
    }
}
