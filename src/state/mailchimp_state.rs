use std::rc::Rc;
use yewdux::store::Reducer;
use yewdux::store::Store;

use super::logging::EventLog;
use super::logging::LoggableEvent;
use super::messages::*;

#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct MailchimpState {
    pub has_been_bugged: bool,
    pub show: bool
}

