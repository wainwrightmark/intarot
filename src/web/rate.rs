use wasm_bindgen::prelude::*;

use crate::state::logging::LoggableEvent;

#[cfg(any(feature = "ios", feature = "android"))]
#[wasm_bindgen()]
extern "C" {
    #[wasm_bindgen(catch, final, js_namespace = ["Capacitor", "Plugins", "RateApp"], js_name="requestReview" )]
    async fn request_review() -> Result<(), JsValue>;
}

#[cfg(any(feature = "ios", feature = "android"))]
pub async fn prompt_user_to_review() {
    let r = request_review().await;

    if let Err(_) = r {
        LoggableEvent::try_log_error_message_async("Failed to request review".to_string()).await;
    }
}
