use leptos::prelude::*;
use leptos::server_fn::serde::{Deserialize, Serialize};
use leptos_meta::*;
use leptos_router::{components::*, path};

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
    pub multiplier: f32,
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
    Text(TextOptions),
    Audio(AudioOptions),
    Video(VideoOptions),
    Image(ImageOptions),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextOptions {
    text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AudioOptions {
    link: String,
    text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoOptions {
    link: String,
    text: String,
    blurred: bool
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageOptions {
    link: String,
    text: String,
    blurred: bool
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    pub question: QuestionType,
    pub answer: String,
    pub multiplier: f32,
}

impl Default for Question {
    fn default() -> Self {
        Self {
            question: QuestionType::Text(TextOptions { text: String::from("Evil?") }),
            answer: "Who is larry?".to_string(),
            multiplier: 1.0,
        }
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            title: "Peenarverse Jeopardy".to_string(),
            categories: vec![Category::default(), Category::default(), Category::default(), Category::default(), Category::default()],
            multiplier: 1.,
        }
    }
}

pub fn create_answered_storage(columns: usize, rows: usize) -> Vec<Vec<bool>> {
    let mut answered: Vec<Vec<bool>> = Vec::new();
    for c in 0..columns {
        answered.push(Vec::<bool>::new());
        for _ in 0..rows {
            answered[c].push(false);
        }
    }

    answered
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="dark" />

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
    use rand::{rng, Rng};
    use std::fs;
    use rand::rngs::ThreadRng;

    #[test]
    fn test_write_game_data() {
        let mut rng = rng();
        let game_data = GameData::random(&mut rng);
        fs::write("questions/questions.json", serde_json::to_string_pretty(&game_data).unwrap()).unwrap();
    }

    impl GameData {
        fn random(rng: &mut ThreadRng) -> Self {
            let mut categories: Vec<Category> = Vec::new();
            let max = rng.random_range(5..11);
            let questions_per_cat = rng.random_range(5..11);
            for _ in 0..max {
                categories.push(Category::random(rng, questions_per_cat));
            }
            Self {
                title: "Randomly Generated".to_string(),
                multiplier: rng.random_range(1.0..2.0),
                categories,
            }
        }
    }

    impl Category {
        fn random(rng: &mut ThreadRng, questions_per_cat: u32) -> Self {
            let mut questions: Vec<Question> = Vec::new();
            let max = rng.random_range(5..11);
            for _ in 0..questions_per_cat {
                questions.push(Question::random(rng));
            }
            Self {
                title: "Randomly Generated".to_string(),
                questions,
            }
        }
    }

    impl Question {
        fn random(rng: &mut ThreadRng) -> Self {
            let multiplier = rng.random_range(1.0..5.0);
            match rng.random_range(0..4) {
                0 => Self {
                    question: QuestionType::Text(TextOptions { text: String::from("Evil?") }),
                    answer: "Who is larry?".to_string(),
                    multiplier,
                },
                1 => Self {
                    question: QuestionType::Audio(AudioOptions {
                        link: String::from("https://static.wikia.nocookie.net/lethalcompanyzeekerss/images/3/3a/Babycry1.mp3"),
                        text: String::from("Whomst maketh this soundy?"),
                    }),
                    answer: "Who is larry?".to_string(),
                    multiplier,
                },
                2 => Self {
                    question: QuestionType::Video(VideoOptions {
                        link: String::from("https://www.youtube.com/embed/IsKyw-_aRdI?si=3O_GMKShwnjoYKxv"),
                        text: String::from("WHompst killed this manz?"),
                        blurred: true,
                    }),
                    answer: "Who is larry?".to_string(),
                    multiplier,
                },
                3 => Self {
                    question: QuestionType::Image(ImageOptions {
                        link: String::from("https://static.wikia.nocookie.net/lethalcompanyzeekerss/images/e/e6/Site-logo.png"),
                        text: String::from("Can you read?"),
                        blurred: true,
                    }),
                    answer: "No, you can't... but larry can".to_string(),
                    multiplier,
                },
                _ => Self {
                    question: QuestionType::Text(TextOptions { text: String::from("Evil?") }),
                    answer: "Who is larry?".to_string(),
                    multiplier,
                }
            }
        }
    }
}
