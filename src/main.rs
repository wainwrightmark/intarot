use crate::state::prelude::*;
use crate::web::prelude::*;

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod data;
pub mod state;
pub mod web;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    wasm_bindgen_futures::spawn_local(ImageMetaState::setup());
    wasm_bindgen_futures::spawn_local(ImageDescriptionState::setup());
    yew::Renderer::<App>::new().render();
}
