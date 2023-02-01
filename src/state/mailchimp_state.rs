use std::rc::Rc;
use yewdux::store::Reducer;
use yewdux::store::Store;





#[derive(PartialEq, Eq, Clone, serde:: Serialize, serde::Deserialize, Store, Debug, Default)]
#[store(storage = "local", storage_tab_sync)]
pub struct MailchimpState {
    pub av_views: usize,
    pub has_been_bugged: bool,
    pub show: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct AVViewMessage;

impl Reducer<MailchimpState> for AVViewMessage {
    fn apply(self, state: Rc<MailchimpState>) -> Rc<MailchimpState> {
        MailchimpState {
            av_views: state.av_views + 1,
            has_been_bugged: state.has_been_bugged,
            show: state.show | (!state.has_been_bugged && state.av_views >= 1),
        }
        .into()
    }
}

impl Reducer<MailchimpState> for ShowMailchimpMessage {
    fn apply(self, state: Rc<MailchimpState>) -> Rc<MailchimpState> {
        MailchimpState {
            show: true,
            av_views: state.av_views,
            has_been_bugged: true,
        }
        .into()
    }
}

impl Reducer<MailchimpState> for HideMailchimpMessage {
    fn apply(self, state: Rc<MailchimpState>) -> Rc<MailchimpState> {
        MailchimpState {
            show: false,
            has_been_bugged: true,
            av_views: state.av_views,
        }
        .into()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ShowMailchimpMessage;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct HideMailchimpMessage;
