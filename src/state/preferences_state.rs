use std::{rc::Rc};

use web_sys::window;
#[cfg(target_arch = "wasm32")]
use yewdux::storage;
use yewdux::{
    prelude::{init_listener, Listener},
    store::Store,
};

#[derive(Default, PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Debug, Store)]
#[store(storage = "local", storage_tab_sync)]
pub enum MotionState {
    #[default]
    FullMotion,
    ReducedMotion
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

fn update_window_dark_state(state: DarkModeState) -> Option<()> {
    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::StatusBar;
        use capacitor_bindings::status_bar::Style;
        let style = match state {
            DarkModeState::Auto => Style::Default,
            DarkModeState::Light => Style::Light,
            DarkModeState::Dark => Style::Dark,
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

    let fixed_class_name = class_name.replace("dark", "").replace("light", "");
    let suffix = match state {
        DarkModeState::Auto => "",
        DarkModeState::Light => " light",
        DarkModeState::Dark => " dark",
    };

    root.set_class_name(format!("{fixed_class_name}{suffix}").as_str());
    Some(())
}

#[derive(Default, PartialEq, Eq, Clone, Copy, serde:: Serialize, serde::Deserialize, Debug)]
pub enum DarkModeState {
    #[default]
    Auto,
    Light,
    Dark,
}


impl Store for DarkModeState {
    #[cfg(not(target_arch = "wasm32"))]
    fn new() -> Self {
        init_listener(DarkModeListener);
        let state: DarkModeState = Default::default();
        update_window_dark_state(state);
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

            update_window_dark_state(state);
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
        update_window_dark_state(*state);

        log::info!("Saving dark mode state {state:?}");
        #[cfg(target_arch = "wasm32")]
        {
            use yewdux::storage::save;
            save(state.as_ref(), storage::Area::Local).expect("unable to save state");
        }
    }
}
