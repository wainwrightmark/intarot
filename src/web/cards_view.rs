use yew::prelude::*;
use yew_hooks::{use_swipe, UseSwipeDirection};
use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::{ShareComponent, Route};

#[derive(Properties, PartialEq, Clone)]
pub struct CardControlProps {
    pub sign: StarSign,
    pub soothsayer: Soothsayer,
    pub ordering: Ordering,
}

#[function_component(CardsControl)]
pub fn cards_control(props: &CardControlProps) -> Html {
    let node = use_node_ref();
    let swipe_state = use_swipe(node.clone());

    let (cp, dispatch) = use_store::<CardPageState>();
    let props = props.clone();
    use_effect(move || {
        dispatch.reduce(|x| x.on_load(props.sign, props.soothsayer, props.ordering))
    });

    {
        let swipe_state = swipe_state.clone();
        use_effect_with_deps(
            move |direction| {
                // Do something based on direction.
                match **direction {
                    UseSwipeDirection::Left => {
                        Dispatch::<CardPageState>::new().apply(DrawMessage {})
                    }
                    UseSwipeDirection::Right => {
                        Dispatch::<CardPageState>::new().apply(ReplaceMessage {})
                    }
                    _ => (),
                }
                || ()
            },
            swipe_state.direction,
        );
    }

    let can_previous = cp.cards_drawn > 1;
    let can_next = cp.cards_drawn < 23;

    let select_previous =
        Dispatch::<CardPageState>::new().apply_callback(move |_| ReplaceMessage {});
    let select_next = Dispatch::<CardPageState>::new().apply_callback(move |_| DrawMessage {});

    let (metas_state, _) = use_store::<ImageMetaState>();
    let (descriptions_state, _) = use_store::<ImageDescriptionState>();

    let Some(ds) = descriptions_state.descriptions.as_ref() else{
        return html!();
    };

    let metas = cp.get_possible_image_metas(metas_state.as_ref());
    let total_cards = cp.cards_drawn + 1; //display an extra card to preload the image
    let s_d: bool = cp.show_description;

    let final_card = if true {
        Some(html!(<FinalCardView total_cards={total_cards} key={"final card"} />))
    } else {
        None
    };

    let items = metas
        .into_iter()
        //.take(cp.cards_drawn)
        .take(cp.max_drawn)
        .enumerate()
        .map(|(index, image_meta)|
        {
            let description = ds[&(image_meta.soothsayer, image_meta.card)].clone();
            let key = image_meta.card.name().clone();
            //let top_card = index + 1 == metas_len;
            html!(<CardView index={index} meta={image_meta} show_description={s_d} description={description} total_cards={total_cards} key={key} />)
        })
        .chain(final_card)
        .collect::<Html>();

    html!(
        <>
        <div class="site" style="overflow: hidden ">
            <div class="container" >

        <div class="sm-4 col" style="margin: auto; width: 90vw; height: 100vh;" ref={node}>
        <div class="cards-grid" key="cards-grid">
        { items }
        </div>
        <div class="card-actions" style="pointer-events: none;">
            <button id="card-button-prev" aria-label="Previous" disabled={!can_previous}  onclick={select_previous} style="pointer-events: auto;" >{"❰"}</button>
            <button id="card-button-next" aria-label="Next"  disabled={!can_next} onclick={select_next}  style="pointer-events: auto;">{"❱"}</button>
        </div>
        </div>
        </div>
        </div>
        </>
    )
}

#[derive(Properties, PartialEq)]
struct FinalCardViewProps {
    pub total_cards: usize,
}

#[function_component(FinalCardView)]
fn final_card_view(props: &FinalCardViewProps) -> Html {
    let card_page_state = use_store_value::<CardPageState>();
    let navigator = use_navigator().unwrap();
    let card_classes = classes!("prophecy-card");

    let top_card = 24 == props.total_cards;

    let style = if top_card {
        let angle = 0;
        format!(
            "transform: rotateZ({}deg); transition-duration: 1s, 3s",
            angle
        )
    } else {
        format!("transform:  translateX(15em) translateY(5em) rotateZ(-30deg); visibility: hidden; pointer-events: none;",)
    };
    let sign = card_page_state.star_sign;
    let soothsayer = card_page_state.soothsayer;

    let shuffle = Callback::from(move |_e: MouseEvent| {
        let ordering = Ordering::gen(22);
        navigator.push(&Route::Card {
            sign,
            soothsayer,
            ordering,
        });
    });

    html! {

            <div class={card_classes} style = {style} >
            <div class="prophecy-back prophecy-image" style="border: solid 2px black; background: antiquewhite;"> </div>
                    <div class="image-overlay" style="pointer-events:none;">
                        <p class="image-overlay-text">
                            <span>
                            {"You have reached the end of the deck"}
                            </span>
                            <br/>
                        </p>
                        <div class="row flex-spaces child-borders" style="margin-top: 3rem; margin-bottom: -3rem;">
                            <button  style="pointer-events:auto;" onclick={shuffle}>{"Shuffle"}</button>
                            <label class="paper-btn margin" for="modal-2"  style="pointer-events:auto;">{"Share"}</label>

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

#[derive(Properties, PartialEq)]
struct CardViewProps {
    pub meta: ImageMeta,
    pub show_description: bool,
    pub index: usize,
    pub description: ImageDescription,
    pub total_cards: usize,
}

#[function_component(CardView)]
fn card_view(props: &CardViewProps) -> Html {
    let toggle = Dispatch::<CardPageState>::new().apply_callback(|_| ToggleDescriptionMessage {});
    // let open_share_dialog = Dispatch::<CardPageState>::new().apply_callback(|_| ToggleShareDialogMessage {});

    let mut card_classes = classes!("prophecy-card");
    let mut image_classes = classes!("prophecy-image");
    let top_card = props.index + 2 == props.total_cards;
    let show_description = if top_card {
        card_classes.push("top_card");

        if props.show_description {
            image_classes.push("image_greyed");
            true
        } else {
            false
        }
    } else {
        false
    };

    let style = if props.index + 1 == props.total_cards {
        format!("transform:  translateX(15em) translateY(5em) rotateZ(-30deg); visibility: hidden; pointer-events: none;",)
    } else if props.index + 1 >= props.total_cards {
        let angle = match props.index % 4 {
            0 => 15 + ((props.index as isize) * -10),
            1 => -20 + ((props.index as isize) * 10),
            2 => 20 + ((props.index as isize) * -10),
            _ => -15 + ((props.index as isize) * 10),
        };

        let translate_x = match props.index % 4 {
            0 => 10,
            1 => -10,
            2 => 20,
            _ => -20,
        };

        let translate_y = match props.index % 4 {
            0 => 10,
            1 => -20,
            2 => -10,
            _ => 20,
        };

        format!(
            "transform:  translateX({}em) translateY({}em) rotateZ({}deg); visibility: hidden; pointer-events: none;",
            translate_x, translate_y, angle,
        )
    } else if top_card {
        let angle = 0;
        format!(
            "transform: rotateZ({}deg); transition-duration: 1s, 3s",
            angle
        )
    } else {
        let angle = match props.index % 4 {
            0 => 15 + ((props.index as isize) * -1),
            1 => -20 + ((props.index as isize) * 1),
            2 => 20 + ((props.index as isize) * -1),
            _ => -15 + ((props.index as isize) * 1),
        };

        let translate_x = match props.index % 4 {
            0 => 1,
            1 => -1,
            2 => 2,
            _ => -2,
        };

        let translate_y = match props.index % 4 {
            0 => 1,
            1 => 0,
            2 => -2,
            _ => -1,
        };

        format!(
            "transform: translateX({}em) translateY({}em)  rotateZ({}deg);s; pointer-events: none;",
            translate_x, translate_y, angle
        )
    };

    html! {

            <div class={card_classes} style = {style} >
            <div class="prophecy-back"> </div>
                    <img class={image_classes}  src={format!("https://drive.google.com/uc?export=view&id={}", props.meta.id.clone()) } onclick={toggle.clone()} />
                    {
                        if show_description{
                            html!{
                                <div class="image-overlay" style="pointer-events:none;">
                                <p class="image-overlay-text">
                                    <span>
                                    {props.description.representation.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {props.description.guidance.clone()}
                                    </span>
                                    <br/>
                                    <br/>
                                    <span>
                                    {props.description.specific_guidance.clone()}
                                    </span>
                                </p>
                                <div class="row flex-spaces child-borders" style="margin-top: 3rem; margin-bottom: -3rem;">
                    <label class="paper-btn margin" for="modal-2"  style="pointer-events:auto;">{"Share"}</label>
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
                                text={props.description.full_description()}
                                media={format!("https://drive.google.com/uc?export=view&id={}", props.meta.id.clone())}>
                                </ShareComponent>

                    </div>
                  </div>
                                </div>
                            }
                        }
                        else{
                            html!{
                                <></>
                            }
                        }
                    }
        </div>
    }
}
