use std::rc::Rc;
use yewdux::prelude::Dispatch;
use yewdux::store::Reducer;
use yewdux::store::Store;

use super::logging::EventLog;
use super::messages::*;
use super::prelude::*;
#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct FailedLogsState {
    pub logs: Vec<LoggableEvent>,
}

impl Reducer<FailedLogsState> for ResentFailedLogsMessage {
    fn apply(self, state: Rc<FailedLogsState>) -> Rc<FailedLogsState> {
        //log::info!("Checking for failed logs");
        if state.logs.is_empty() {
            return state;
        }
        let user = Dispatch::<UserState>::new().get();
        let Some(user_id) = user.user_id else{
            log::error!("User Id not set");
            return state;
        };

        log::info!("{} failed logs found", state.logs.len());

        for event in state.logs.iter() {
            let log = EventLog {
                user_id,
                resent: true,
                event: event.clone(),
            };
            log.send_log();
        }

        FailedLogsState::default().into()
    }
}

impl Reducer<FailedLogsState> for LogFailedMessage {
    fn apply(self, state: Rc<FailedLogsState>) -> Rc<FailedLogsState> {
        let mut new_state = (*state).clone();
        new_state.logs.push(self.0);

        new_state.into()
    }
}
