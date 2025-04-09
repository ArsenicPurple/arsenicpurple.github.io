use leptos::prelude::*;
use leptos::server_fn::serde::{Deserialize, Serialize};
use leptos_meta::*;
use leptos_router::{components::*, path};
use std::future::Future;


// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    Home,
    Question((usize, usize)),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameData {
    pub title: String,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub title: String,
    pub questions: Vec<Question>,
}

impl Default for Category {
    fn default() -> Self {
        Self {
            title: "Fard".to_string(),
            questions: vec![Question::default(), Question::default(), Question::default(), Question::default(), Question::default()] }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum QuestionType {
    Text(String),
    Audio(String, String),
    Video(String, String),
    Image(String, String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    pub answered: bool,
    pub question: QuestionType,
    pub answer: String,
}

impl Default for Question {
    fn default() -> Self {
        // let mut rng = rng();
        // match rng.random_range(0..4) {
        //     0 => Self {
        //         answered: false,
        //         question: QuestionType::Text(String::from("Evil?")),
        //         answer: "Who is larry?".to_string(),
        //     },
        //     1 => Self {
        //         answered: false,
        //         question: QuestionType::Audio(String::from("https://static.wikia.nocookie.net/lethalcompanyzeekerss/images/3/3a/Babycry1.mp3"), String::from("Whomst maketh this soundy?")),
        //         answer: "Who is larry?".to_string(),
        //     },
        //     2 => Self {
        //         answered: false,
        //         question: QuestionType::Video(String::from("https://www.youtube.com/embed/IsKyw-_aRdI?si=3O_GMKShwnjoYKxv"), String::from("WHompst killed this manz?")),
        //         answer: "Who is larry?".to_string(),
        //     },
        //     3 => Self {
        //         answered: false,
        //         question: QuestionType::Image(String::from("https://static.wikia.nocookie.net/lethalcompanyzeekerss/images/e/e6/Site-logo.png"), String::from("Can you read?")),
        //         answer: "No, you can't... but larry can".to_string(),
        //     },
        //     _ => Self {
        //         answered: false,
        //         question: QuestionType::Text(String::from("Evil?")),
        //         answer: "Who is larry?".to_string(),
        //     }
        // }
        Self {
            answered: false,
            question: QuestionType::Text(String::from("Evil?")),
            answer: "Who is larry?".to_string(),
        }
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            title: "Peenarverse Jeopardy".to_string(),
            categories: vec![Category::default(), Category::default(), Category::default(), Category::default(), Category::default()],
        }
    }
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Welcome to Leptos CSR" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_write_game_data() {
        let game_data = GameData::default();
        fs::write("questions/questions.json", serde_json::to_string_pretty(&game_data).unwrap()).unwrap();
    }
}
