use std::rc::Rc;

use yewdux::prelude::async_reducer;
use yewdux::store::AsyncReducer;
use yewdux::store::Store;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct NaggingState {
    pub has_submitted_email: bool,
    pub advanced_visits: u32,
}

pub struct AdvancedPageVisitMessage;

#[async_reducer]
impl AsyncReducer<NaggingState> for AdvancedPageVisitMessage {
    async fn apply(self, mut nagging_state: Rc<NaggingState>) -> Rc<NaggingState> {
        let state = Rc::make_mut(&mut nagging_state);
        state.advanced_visits += 1;

        #[cfg(any(feature = "ios", feature = "android"))]
        {
            if state.advanced_visits > 4 {
                crate::web::rate::prompt_user_to_review().await;
            }
        }

        #[cfg(feature = "web")]
        {
            if !state.has_submitted_email && state.advanced_visits.is_power_of_two() {
                let info_result = capacitor_bindings::device::Device::get_info().await;

                let Ok(info) = info_result else{ return nagging_state;};

                if info.platform == capacitor_bindings::device::Platform::IOs {
                    if state.advanced_visits > 3 {
                        loop {
                            let r#break = nag_for_email(state).await;
                            if r#break {
                                break;
                            }
                        }
                    }
                } else {
                    nag_for_app_store().await;
                    return nagging_state;
                }
            }
        }

        nagging_state
    }
}

#[cfg(feature = "web")]
async fn nag_for_app_store() {
    use super::logging::LoggableEvent;
    use capacitor_bindings::action_sheet::ActionSheetButton;
    use capacitor_bindings::action_sheet::ShowActionsOptions;
    use web_sys::window;
    let show_result =
        capacitor_bindings::action_sheet::ActionSheet::show_actions(ShowActionsOptions {
            title: "Looking for the Android App?".to_string(),
            message: None,
            options: vec![
                ActionSheetButton {
                    title: "Get it on Google Play".to_string(),
                    icon: None,
                    style: Some(capacitor_bindings::action_sheet::ActionSheetButtonStyle::Default),
                },
                ActionSheetButton {
                    title: "Cancel".to_string(),
                    icon: None,
                    style: Some(capacitor_bindings::action_sheet::ActionSheetButtonStyle::Cancel),
                },
            ],
        })
        .await;

    match show_result {
        Ok(a) => {
            if a.index == 0 {
                LoggableEvent::try_log_async(LoggableEvent::GoToAppStore).await;
                let _ = window()
                    .unwrap()
                    .location()
                    .replace("https://play.google.com/store/apps/details?id=com.intarot.app");
            } else {
                LoggableEvent::try_log_async(LoggableEvent::NoGoToAppStore).await;
            }
        }
        Err(e) => LoggableEvent::try_log_error_message_async(e.to_string()).await,
    }
}

#[cfg(feature = "web")]
async fn nag_for_email(state: &mut NaggingState) -> bool {
    use super::logging::LoggableEvent;

    let signup_result = crate::web::modal_dialog::prompt_dialog(
        "Enter your email to be notified when we're on the app store".to_string(),
        "Mobile App Coming Soon".to_string(),
        "Submit".to_string(),
        "No Thanks".to_string(),
        "Your Email".to_string(),
    )
    .await;

    match signup_result {
        Ok(prompt_result) => {
            let Some(email) =prompt_result.as_string() else{
                LoggableEvent::try_log_async(
                    LoggableEvent::NoSubmitEmail {advanced_visits: state.advanced_visits }).await;
                return true;};

            if !email.contains('@') {
                log::warn!("Bad Email {email}");
                return false; //Did not pass my sophisticated email validation
            }

            // let client = reqwest::Client::new();
            // let response = client

            //     .post(MAILCHIMP_URL)
            //     .header("Content-Type", "Content-Type: application/json")
            //     .form(&FormData {
            //         email: email.clone(),
            //     })
            //     .send()
            //     .await;

            LoggableEvent::try_log_async(LoggableEvent::SubmitEmail {
                address: email,
                advanced_visits: state.advanced_visits,
            })
            .await;
            return true;
        }
        Err(err) => {
            let err = capacitor_bindings::helpers::Error::from(err);
            LoggableEvent::try_log_error_message_async(err.to_string()).await;
            return true;
        }
    }
}

// const MAILCHIMP_URL: &'static str = r#"https://app.us21.list-manage.com/subscribe/post-json?u=b9e9d767448b6f3485a30c05b&amp;id=6234ad4091&amp;f_id=00d5dee1f0"#;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct FormData {
    pub email: String,
}
