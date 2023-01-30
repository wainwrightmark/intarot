use std::collections::BTreeSet;
use std::rc::Rc;
use yewdux::prelude::Dispatch;
use yewdux::store::Reducer;
use yewdux::store::Store;

use super::logging::EventLog;
use super::messages::*;
use super::user_state::UserState;
use crate::data::prelude::*;
use crate::state::prelude::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct AchievementsState {
    pub achieved: BTreeSet<Achievement>,
}

impl Reducer<AchievementsState> for AchievementEarnedMessage {
    fn apply(self, state: Rc<AchievementsState>) -> Rc<AchievementsState> {
        if state.achieved.contains(&self.0) {
            return state;
        }

        let mut new_state = (*state).clone();
        new_state.achieved.insert(self.0);

        let user = Dispatch::<UserState>::new().get();
        if let Some(user_id) = user.user_id {
            let message = EventLog::new(user_id, self.0.into());
            message.send_log();
        } else {
            Dispatch::<FailedLogsState>::new().apply(LogFailedMessage(self.0.into()));
            log::error!("User Id not set");
        }

        Rc::new(new_state)
    }
}
