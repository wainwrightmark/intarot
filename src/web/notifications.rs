use capacitor_bindings::local_notifications::*;
use yewdux::prelude::Dispatch;

use crate::state::prelude::*;

pub async fn setup_notifications_async() {
    let schedule_options = ScheduleOptions {
        notifications: vec![LocalNotificationSchema {
            auto_cancel: true,
            body: "Your Daily Reading is ready".to_string(),
            title: "Your daily reading".to_string(),
            schedule: Schedule {
                on: ScheduleOn {
                    second: None, // Some(0),// None,
                    year: None,
                    minute: None,
                    month: None,
                    day: None,
                    weekday: None,
                    hour: Some(9), //Trigger at 9am each day
                },
                allow_while_idle: true,
            },
            large_body: None,
            summary_text: Some("Your Daily Reading is ready".to_string()),
            id: -1125158782, //Very Random number
            ongoing: false,
            inbox_list: None,
            action_type_id: Some("DailyReading".to_string()),
            small_icon: Some("icon512".to_string()),
            large_icon: Some("splash".to_string()),
            icon_color: Some("#000000".to_string()),
            group: None,
            group_summary: None,
        }],
    };

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

    log::info!("Registering Action Types");
    let action_type_options = RegisterActionTypesOptions {
        types: vec![ActionType {
            id: "DailyReading".to_string(),
            actions: vec![Action {
                id: "ViewReading".to_string(),
                title: "View Reading".to_string(),
            }],
        }],
    };
    LocalNotifications::register_action_types(action_type_options).await;

    schedule_notification(schedule_options, on_action).await;
}

async fn schedule_notification<F: Fn(ActionPerformed) + 'static>(
    schedule_options: ScheduleOptions,
    on_action: F,
) -> () {
    log::info!("Scheduling local notification...");
    let result = LocalNotifications::schedule(schedule_options).await;

    log::info!("Notification Scheduled {:?}", result.notifications);

    log::info!("Registering Action Listener");
    LocalNotifications::add_action_performed_listener(on_action).await;
    log::info!("Action Listener Registered");
}
