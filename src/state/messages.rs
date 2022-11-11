use yew::{Classes, Html};
use yewdux::store::Reducer;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ResetMessage {}
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ProceedMessage {}
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct DrawMessage {}
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ShuffleMessage {}
#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct ToggleDescriptionMessage {}

pub trait ButtonMessage<S>: Reducer<S> + Default + Copy + Clone {
    /// Whether this message can be applied to the state
    fn can_apply(state: &S) -> bool;

    /// Button Text
    fn get_name() -> &'static str;
}

pub trait SelectCarouselMessage<S>: Reducer<S> + Copy + Clone + PartialEq {
    fn get_html(&self, classes: Classes) -> Html;

    fn get_values() -> Vec<Self>;
    fn get_current_value(state: &S) -> Self;
}

pub trait SelectMessage<S>: Reducer<S> + Copy + Clone + PartialEq {
    fn get_values() -> Vec<Self>;
    fn get_current_value(state: &S) -> Self;

    fn disabled(&self) -> bool;

    fn parse_repr(s: &str) -> Self;

    fn repr(&self) -> &'static str;

    fn name(&self) -> &'static str;
}
