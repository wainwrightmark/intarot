use rand::rngs::ThreadRng;
use rand::RngCore;
use yew::prelude::*;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;


use crate::state::prelude::*;
use crate::web::prelude::{Route, ShareComponent};

#[derive(Properties, PartialEq)]
pub struct FinalCardViewProps {
    pub total_cards: usize,
}

#[function_component(FinalCardView)]
pub fn final_card_view(props: &FinalCardViewProps) -> Html {
    let card_page_state = use_store_value::<CardPageState>();
    let navigator = use_navigator().unwrap();
    let card_classes = classes!("prophecy-card");
    let image_classes = classes!("prophecy-image");

    let top_card = 24 == props.total_cards;

    let style = if top_card {
        let angle = 0;
        format!(
            "transform: rotateZ({angle}deg); transition-duration: 1s, 3s"
        )
    } else {
        "transform:  translateX(15em) translateY(5em) rotateZ(-30deg); visibility: hidden; pointer-events: none;".to_string()
    };
    let sign = card_page_state.star_sign;
    let guide = card_page_state.guide;

    let shuffle = Callback::from(move |_e: MouseEvent| {
        let seed = ThreadRng::default().next_u32();
        navigator.push(&Route::Card {
            sign: sign.into(),
            guide,
            seed,
        });
    });

    html! {

            <div class={card_classes} style = {style} >
                <div class="prophecy-back"> </div>
                <img class={image_classes}  src={format!("https://drive.google.com/uc?export=view&id={}", guide.ad_image_id()) } />
                <div class="image-overlay" style="pointer-events:none;">
                    <p class="image-overlay-text">
                        // <span>
                        // {"You have reached the end of the deck"}
                        // </span>
                        <br/>
                    </p>
                    <div class="row flex-spaces child-borders" style="margin-top: 13rem; margin-bottom: -3rem; flex-direction: column;">
                        <button class="nice-button"  style="pointer-events:auto;" onclick={shuffle}>{"Shuffle"}</button>
                        <label class="paper-btn margin nice-button" for="modal-2"  style="pointer-events:auto;">{"Share"}</label>

                    </div>
                    <br/>
                    <input class="modal-state" id="modal-2" type="checkbox"/>
                    <div class="modal" style="pointer-events:auto;">
                            <label class="modal-bg" for="modal-2"></label>
                        <div class="modal-body">
                        <h4 class="modal-title">{"Share"}</h4>
                        <ShareComponent
                        title="intarot"
                        url={"https://www.intarot.com"}
                        text={ include_str!(r#"../text/opening_p1.txt"#)}
                        media={""}
                        // media={format!("https://drive.google.com/uc?export=view&id={}", props.meta.id.clone())}
                        >
                        </ShareComponent>
                    </div>
                </div>
                </div>
            </div>
    }
}
