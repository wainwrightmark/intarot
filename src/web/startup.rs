use crate::state::failed_logs_state::FailedLogsState;
use crate::state::prelude::*;
use web_sys::window;
use yewdux::prelude::Dispatch;

pub async fn setup(ref_param: Option<String>, gclid_param: Option<String>) {

    Dispatch::<ImageMetaState>::new()
        .apply_future(SetUpImageMetaStateMessage)
        .await;

    Dispatch::<UserState>::new()
        .apply_future(UpdateParamsIfNewMessage {
            ref_param,
            gclid_param,
        })
        .await;
    Dispatch::<FailedLogsState>::new()
        .apply_future(ResentFailedLogsMessage)
        .await;

    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error_async(|| async {
            StatusBar::set_overlays_web_view(SetOverlaysWebViewOptions { overlay: true }).await
        })
        .await;

        let style = if is_dark_mode() { //TODO watch dark mode changes
            Style::Dark
        } else {
            Style::Light
        };
        crate::web::capacitor::do_or_report_error_async(move || async move {
            StatusBar::set_style(style).await
        })
        .await;

        match capacitor_bindings::app::App::add_back_button_listener(move |event| {
            if !(event.can_go_back && try_go_back()) {
                crate::web::capacitor::do_or_report_error(
                    capacitor_bindings::app::App::minimize_app,
                );
            }
        })
        .await
        {
            Ok(handle) => handle.leak(),
            Err(err) => {
                crate::state::logging::LoggableEvent::try_log_error_message_async(err.to_string())
                    .await;
            }
        }
    }
    #[cfg(feature = "ios")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error_async(|| async { StatusBar::hide().await }).await;
    }

    crate::setup_notifications_async().await;
}

fn is_dark_mode() -> bool {
    let Some(window) = window() else{return false;};
    match window.match_media("(prefers-color-scheme: dark)") {
        Ok(list) => match list {
            Some(mql) => mql.matches(),
            None => false,
        },
        Err(_) => false,
    }
}

/// Goes back, returns true if successful
fn try_go_back() -> bool {
    match window() {
        Some(w) => match w.history() {
            Ok(h) => match h.back() {
                Ok(_) => true,
                Err(_) => false,
            },
            Err(_) => false,
        },
        None => false,
    }
}
