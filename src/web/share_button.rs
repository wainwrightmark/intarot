use crate::data::prelude::*;
use crate::state::prelude::*;
use capacitor_bindings::share::Share;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ShareButtonProps {
    pub label: Option<AttrValue>,
    pub src_data: SrcData,
    pub share_text: AttrValue,
}

async fn share(text: AttrValue, src_data: SrcData) {

    LoggableEvent::try_log(src_data.clone());

    let result = Share::share(capacitor_bindings::share::ShareOptions{
        title: Some("intarot".to_string()),
        text: Some(text.to_string()),
        url: Some(src_data.share_url()),
        dialog_title: Some("intarot".to_string()),
        files: None,
    }).await;



    LoggableEvent::try_log(LoggableEvent::ShareOn { platform: result.activity_type });
}

#[function_component(ShareButton)]
pub fn share_button(props: &ShareButtonProps) -> Html {
    let src_data: SrcData = props.src_data.clone();
    let text = props.share_text.clone();

    let on_click = move |_: MouseEvent| {
        let future = share(text.clone(), src_data.clone());

        wasm_bindgen_futures::spawn_local(future);
    };

    if let Some(label) = &props.label {
        html!(<button class="paper-btn nice-button card-button" onclick={on_click}>{label.clone()}</button>)
    } else {
        html!(<button class="" onclick={on_click} style="border:none; background: transparent; pointer-events:auto;"><ShareIcon /></button>)
    }
}

#[function_component(ShareIcon)]
pub fn share_icon() -> Html {
    html!(
            <svg fill="#000000" width="24px" height="24px" viewBox="0 0 483 483">
    <g>
        <path d="M395.72,0c-48.204,0-87.281,39.078-87.281,87.281c0,2.036,0.164,4.03,0.309,6.029l-161.233,75.674
		c-15.668-14.971-36.852-24.215-60.231-24.215c-48.204,0.001-87.282,39.079-87.282,87.282c0,48.204,39.078,87.281,87.281,87.281
		c15.206,0,29.501-3.907,41.948-10.741l69.789,58.806c-3.056,8.896-4.789,18.396-4.789,28.322c0,48.204,39.078,87.281,87.281,87.281
		c48.205,0,87.281-39.078,87.281-87.281s-39.077-87.281-87.281-87.281c-15.205,0-29.5,3.908-41.949,10.74l-69.788-58.805
		c3.057-8.891,4.789-18.396,4.789-28.322c0-2.035-0.164-4.024-0.308-6.029l161.232-75.674c15.668,14.971,36.852,24.215,60.23,24.215
		c48.203,0,87.281-39.078,87.281-87.281C482.999,39.079,443.923,0,395.72,0z"/>
    </g>
    </svg>
        )
}
