use super::logging::LoggableEvent;
use super::messages::*;
use crate::data::prelude::*;
use std::collections::BTreeSet;
use std::rc::Rc;
use yewdux::store::Reducer;
use yewdux::store::Store;

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

        LoggableEvent::try_log(self.0);

        Rc::new(new_state)
    }
}
