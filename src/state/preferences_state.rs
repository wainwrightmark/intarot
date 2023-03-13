use std::rc::Rc;

use web_sys::window;
#[cfg(target_arch = "wasm32")]
use yewdux::storage;
use yewdux::{
    prelude::{init_listener, Listener},
    store::{Reducer, Store},
};

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Debug, Store)]
#[store(storage = "local", storage_tab_sync)]
pub struct CardShakeState {
    pub enabled: bool,
}

impl Default for CardShakeState {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CardShakeToggleMessage;
impl Reducer<CardShakeState> for CardShakeToggleMessage {
    fn apply(self, mut state: Rc<CardShakeState>) -> Rc<CardShakeState> {
        let s = Rc::make_mut(&mut state);
        s.enabled = !s.enabled;
        state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DarkModeToggleMessage;
impl Reducer<DarkModeState> for DarkModeToggleMessage {
    fn apply(self, mut state: Rc<DarkModeState>) -> Rc<DarkModeState> {
        let s = Rc::make_mut(&mut state);
        s.is_dark = !s.is_dark;
        state
    }
}

pub fn is_media_prefers_dark() -> bool {
    let Some(window) = window() else{return false;};
    match window.match_media("(prefers-color-scheme: dark)") {
        Ok(list) => match list {
            Some(mql) => mql.matches(),
            None => false,
        },
        Err(_) => false,
    }
}

fn set_dark(dark: bool) -> Option<()> {
    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::StatusBar;
        use capacitor_bindings::status_bar::Style;
        let style = if dark {
            //TODO watch dark mode changes
            Style::Dark
        } else {
            Style::Light
        };
        crate::web::capacitor::do_or_report_error(move || async move {
            StatusBar::set_style(style).await
        });
    }

    log::info!("Setting Dark");

    let window = window()?;
    let document = window.document()?;
    let body = document.body()?;
    let root = body.parent_element()?;
    let class_name = root.class_name();
    let contains_dark = class_name
        .split_ascii_whitespace()
        .any(|x| x.eq_ignore_ascii_case("dark"));

    log::info!("Setting Dark 2");

    if dark {
        if contains_dark {
        } else {
            root.set_class_name(format!("{class_name} dark").as_str());
        }
    } else if contains_dark {
        root.set_class_name(class_name.replace("dark", "").trim())
    }
    Some(())
}

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Debug)]
pub struct DarkModeState {
    pub is_dark: bool,
}

impl Default for DarkModeState {
    fn default() -> Self {
        let is_dark = is_media_prefers_dark();
        Self { is_dark }
    }
}

impl Store for DarkModeState {
    #[cfg(not(target_arch = "wasm32"))]
    fn new() -> Self {
        init_listener(DarkModeListener);
        let state: DarkModeState = Default::default();
        set_dark(state.is_dark);
        state
    }

    #[cfg(target_arch = "wasm32")]

    fn new() -> Self {
        log::info!("Loading Dark Mode State");
        init_listener(DarkModeListener);

        let state: DarkModeState = storage::load(storage::Area::Local)
            .ok()
            .flatten()
            .unwrap_or_default();

        set_dark(state.is_dark);
        state
    }

    fn should_notify(&self, other: &Self) -> bool {
        self != other
    }
}

struct DarkModeListener;
impl Listener for DarkModeListener {
    type Store = DarkModeState;

    fn on_change(&mut self, state: Rc<Self::Store>) {
        set_dark(state.is_dark);

        log::info!("Saving dark mode state {state:?}");
        #[cfg(target_arch = "wasm32")]
        {
            use yewdux::storage::save;
            save(state.as_ref(), storage::Area::Local).expect("unable to save state");
        }
    }
}
