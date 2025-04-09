use leptos::html::Input;
use leptos::IntoView;
use leptos::leptos_dom::log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::js_sys::{ArrayBuffer, Uint8Array};
use web_sys::{Blob, HtmlInputElement};
use crate::GameData;

#[component]
pub fn Controls() -> impl IntoView {
    let read_game_data = use_context::<ArcReadSignal<GameData>>().expect("Error");
    let set_game_data = use_context::<ArcWriteSignal<GameData>>().expect("Error");
    view! {
        <div id="header-filler">
        </div>
        <div id="title-container">
            <h1>{ move || read_game_data.get().title.clone()}</h1>
        </div>
        <div id="file-container">
            <label id="file-input-label" for="file-input">"Import Board"</label>
            <input
                type="file"
                id="file-input"
                on:input=move |ev| {
                    let input = ev.target().unwrap().unchecked_into::<HtmlInputElement>();
                    if let Some(files) = input.files() {
                        if let Some(file) = files.item(0) {
                            if let Ok(blob) = file.slice() {
                                spawn_local({
                                    let sgd_copy = set_game_data.clone();
                                    async move {
                                        let game_data = parse_file_blob(blob).await;
                                        sgd_copy.set(game_data);
                                    }
                                })
                            }
                        }
                    }
                }
            />
        </div>
    }
}

async fn parse_file_blob(blob: Blob) -> GameData {
    let file_raw_data = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
        .await
        .expect("File reading should not fail");
    let file_raw_data = file_raw_data
        .dyn_into::<ArrayBuffer>()
        .expect("Expected an ArrayBuffer");
    let file_raw_data = Uint8Array::new(&file_raw_data);
    let mut file_bytes = vec![0; file_raw_data.length() as usize];
    file_raw_data.copy_to(file_bytes.as_mut_slice());
    serde_json::from_slice::<GameData>(file_bytes.as_slice()).expect("File deserialization should not fail")
}