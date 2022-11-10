use wasm_bindgen_futures::JsFuture;

use crate::web::prelude::*;
pub mod web;

fn main() {
    wasm_logger::init(wasm_logger::Config::default()); 

    wasm_bindgen_futures::spawn_local(ImageMetaState::setup());
    wasm_bindgen_futures::spawn_local(ImageDescriptionState::setup());
    yew::Renderer::<App>::new().render();
}
