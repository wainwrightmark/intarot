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

        #[cfg(feature = "web")]
        {
            use super::logging::LoggableEvent;
            if !state.has_submitted_email
                && state.advanced_visits > 1
                && state.advanced_visits.is_power_of_two()
            {
                loop {
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
                                break;};

                            if !email.contains('@') {
                                log::warn!("Bad Email {email}");
                                continue; //Did not pass my sophisticated email validation
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
                            break;
                        }
                        Err(err) => {
                            let err = capacitor_bindings::helpers::Error::from(err);
                            LoggableEvent::try_log_error_message_async(err.to_string()).await;
                            break;
                        }
                    }
                }
            }
        }

        nagging_state
    }
}

// const MAILCHIMP_URL: &'static str = r#"https://app.us21.list-manage.com/subscribe/post-json?u=b9e9d767448b6f3485a30c05b&amp;id=6234ad4091&amp;f_id=00d5dee1f0"#;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub struct FormData {
    pub email: String,
}
