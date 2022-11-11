use yew::prelude::*;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::button_component::ButtonComponent;

#[function_component(CardsControl)]
pub fn cards_control() -> Html {
    html!(
        <>
        <div class="sm-4 col" style="margin: auto; width: 100em;">
        <CardsView />
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <ButtonComponent<DrawMessage, PageState> />
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <ButtonComponent<ShuffleMessage, PageState> />
        </div>
        <div class="sm-4 col" style="margin: auto; width: 10em;">
        <ButtonComponent<ResetMessage, PageState> />
        </div>
        </>
    )
}

#[function_component(CardsView)]
pub fn cards_view() -> Html {
    let (metas_state, _) = use_store::<ImageMetaState>();
    let (page_state, _) = use_store::<PageState>();

    let PageState::CardPage(cp) = page_state.as_ref() else{
        return html!();
    };

    let metas = cp.get_image_meta(metas_state.as_ref());
    let s_d: bool = cp.show_description;
    let items = metas
        .into_iter()
        .map(|image_meta| html!(<CardView meta={image_meta} show_description={s_d} />))
        .collect::<Html>();
    html!({ items })
}

#[derive(Properties, PartialEq)]
pub struct CardViewProps {
    pub meta: ImageMeta,
    pub show_description: bool,
}

#[function_component(CardView)]
pub fn card_view(props: &CardViewProps) -> Html {
    let (description_state, _) = use_store::<ImageDescriptionState>();

    let toggle = Dispatch::<PageState>::new().apply_callback(|_| ToggleDescriptionMessage {});

    let image_classes = if props.show_description {
        classes!("prophecy-image-front", "image_greyed")
    } else {
        classes!("prophecy-image-front")
    };

    let Some(description) = description_state.descriptions.as_ref()
    .map(|map|map
        .get(&(props.meta.soothsayer, props.meta.card)))
        .flatten() else{
        return html!(<></>);
    };

    html! {
            <div class="grid">
            <div class="prophecy-image-container">
                <div class="prophecy-image-card" key={props.meta.id.clone()}>
                    <img class={image_classes}  src={format!("https://drive.google.com/uc?export=view&id={}", props.meta.id.clone()) } onclick={toggle} />
                    {
                        if props. show_description{
                            html!{
                                <p class="image-overlay">
                                    <span>
                                    {description.representation.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {description.guidance.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {description.specific_guidance.clone()}
                                    </span>
                                </p>
                            }
                        }
                        else{
                            html!{
                                <></>
                            }

                        }
                    }
                    <div class="prophecy-image-back"/>
                    <div class="prophecy-image-ghost"/>
                </div>

                </div>
    </div>
        }
}
