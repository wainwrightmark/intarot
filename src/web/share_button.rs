use yew::prelude::*;
use crate::data::prelude::*;
use crate::web::share_component::ShareComponent;

#[derive(Properties, PartialEq)]
pub struct ShareButtonProps{
    pub label: AttrValue,
    pub share_text: AttrValue,
    pub src_data: SrcData
}

#[function_component(ShareButton)]
pub fn share_button(props: &ShareButtonProps) -> Html {

    html!(
        <>
        <label class="paper-btn margin nice-button stick-to-bottom" for="modal-2"  style="pointer-events:auto;">{props.label.clone()}</label>
        <input class="modal-state" id="modal-2" type="checkbox"/>
        <div class="modal" style="pointer-events:auto;">
          <label class="modal-bg" for="modal-2"></label>
          <div class="modal-body">
            <h4 class="modal-title">{"Share" }</h4>
                      <ShareComponent
                      title="intarot"
                      url={props.src_data.share_url()}
                      text={props.share_text.clone()}
                      media={props.src_data.src()}>
                      </ShareComponent>

              </div>
      </div>
      </>
    )
}