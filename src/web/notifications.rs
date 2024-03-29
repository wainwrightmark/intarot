use capacitor_bindings::local_notifications::*;
use yewdux::prelude::Dispatch;

use crate::state::prelude::*;

pub async fn setup_notifications_async() {
    let schedule_options = LocalNotificationSchema::builder()
        .title("Your Daily Reading")
        .body("Start your day with a mindful moment")
        .summary_text("Start your day with a mindful moment")
        .id(-1125158782)//Very Random number
        .action_type_id("DailyReading")
        .small_icon("icon512")
        .large_icon("splash")
        .icon_color("#000000")
        .schedule(ScheduleOn::builder().hour(8).build())
        .auto_cancel(true)
        .build();

    let on_action = move |action: ActionPerformed| {
        if action.action_id == "ViewReading" || action.action_id == "tap" {
            Dispatch::<DataState>::new().apply(ChangeSpreadTypeMessage(
                crate::data::prelude::SpreadType::DayAhead,
            ));
            let event = LoggableEvent::ViewDailyReading {};
            LoggableEvent::try_log(event);

            web_sys::window()
                .expect("Could not get window")
                .open_with_url("/question")
                .expect("Could not open question page");
        }
    };

    #[cfg(any(feature = "ios", feature = "android"))]
    {
        log::info!("Registering Action Types");

        crate::web::capacitor::do_or_report_error_async(|| {
            let action_type_options = RegisterActionTypesOptions {
                types: vec![ActionType {
                    id: "DailyReading".to_string(),
                    actions: vec![Action {
                        id: "ViewReading".to_string(),
                        title: "View Reading".to_string(),
                    }],
                }],
            };
            LocalNotifications::register_action_types(action_type_options)
        })
        .await;
    }

    schedule_notification(schedule_options, on_action).await;
}

async fn schedule_notification<F: Fn(ActionPerformed) + 'static>(
    schedule_options: impl Into<ScheduleOptions>,
    on_action: F,
) {
    log::info!("Scheduling local notification...");
    let schedule_result = LocalNotifications::schedule(schedule_options).await;

    match schedule_result {
        Ok(sr) => {
            log::info!("Notification Scheduled {:?}", sr.notifications);

        }
        Err(err) => {
            LoggableEvent::try_log_error_message_async(err.to_string()).await;
        }
    }

    log::info!("Registering Action Listener");
    let listener_result = LocalNotifications::add_action_performed_listener(on_action).await;
    match listener_result {
        Ok(lr) => {
            lr.leak();
        }
        Err(err) => {
            LoggableEvent::try_log_error_message_async(err.to_string()).await;
        }
    }
    log::info!("Action Listener Registered");
}
