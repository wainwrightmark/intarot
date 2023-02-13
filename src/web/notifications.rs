use capacitor_bindings::local_notifications::*;

pub fn setup_notifications(){
    let options = ScheduleOptions {
        notifications: vec![LocalNotificationSchema {
            auto_cancel: true,
            body: "Your Daily Reading is ready".to_string(),
            title: "Your daily reading".to_string(),
            schedule: Schedule {
                on: ScheduleOn {
                    second: Some(0),
                    year: None,
                    minute: None,
                    month: None,
                    day: None,
                    weekday: None,
                    hour: None,
                },
                allow_while_idle: true,
            },
            large_body: Some("Notification Large Body".to_string()),
            summary_text: Some("Notification Summary Text".to_string()),
            id: -1125158782, //Very Random number
            ongoing: false,
            inbox_list: vec![
                // "N One".to_string(),
                // "N Two".to_string(),
                // "N Three".to_string(),
                // "N Four".to_string(),
                // "N Five".to_string(),
            ],
        }],
    };

    wasm_bindgen_futures::spawn_local(schedule_notification(options));
}



async fn schedule_notification(options: ScheduleOptions) -> () {
    log::info!("Scheduling local notification...");
    log::info!("{options:?}");
    let result = LocalNotifications::schedule(&options).await;

    log::info!("Notification Scheduled {:?}", result.notifications);
}