use std::rc::Rc;
use yewdux::store::Reducer;
use yewdux::store::Store;

use super::messages::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local")]
pub struct UserState{
    pub has_used_before : bool
}

impl Reducer<UserState> for SetUsedBeforeMessage{
    fn apply(self, state: Rc<UserState>) -> Rc<UserState> {
        if state.has_used_before{state}
        else{
            UserState{
                has_used_before: true
            }.into()
        }
    }
}