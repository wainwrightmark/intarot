use crate::state::failed_logs_state::FailedLogsState;
use crate::state::prelude::*;
use yewdux::prelude::Dispatch;

#[cfg(any(feature = "ios", feature = "android"))]
use capacitor_bindings::app::{App};

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

    #[cfg(any(feature = "ios", feature = "android"))]
    {
        LoggableEvent::try_log(LoggableEvent::Internal {
            message: format!("Launched on mobile"),
        });
        if let Ok(Some(url)) = App::get_launch_url().await {
            LoggableEvent::try_log(LoggableEvent::Internal {
                message: format!("app launch url: {url:?}"),
            });
            let url = url.url;
            if let Some(window) = web_sys::window() {
                if let Ok(href) = window.location().href() {
                    const PREFIX: &'static str = "https://intarot.app/";
                    if url.to_ascii_lowercase().starts_with(PREFIX) {
                        let (_, url_suffix) = url.split_at(PREFIX.len());

                        let fixed_url = "http://localhost/".to_string() + url_suffix;

                        if href != fixed_url {
                            LoggableEvent::try_log(LoggableEvent::Internal {
                                message: format!(
                                    "Should route: current: {href}, path: {url} fixed: {fixed_url}"
                                ),
                            });
                            let _  = window.location().set_href(&fixed_url);
                        }
                    }
                }
            }
        }
    }

    #[cfg(feature = "android")]
    {
        use capacitor_bindings::status_bar::*;
        crate::web::capacitor::do_or_report_error_async(|| async {
            StatusBar::set_overlays_web_view(SetOverlaysWebViewOptions { overlay: true }).await
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

#[cfg(feature = "android")]
/// Goes back, returns true if successful
fn try_go_back() -> bool {
    use web_sys::window;
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
