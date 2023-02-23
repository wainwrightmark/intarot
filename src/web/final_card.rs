use yew::prelude::*;

use yew_router::prelude::use_navigator;
use yewdux::prelude::*;

use crate::data::prelude::*;
use crate::state::prelude::*;
use crate::web::prelude::Route;
use crate::web::prelude::*;

use super::card_view::CardStyle;

#[derive(Properties, PartialEq)]
pub struct FinalCardProps {
    pub top_card: bool,
    pub style: CardStyle,
    pub src_data: SrcData,
}

#[function_component(FinalCard)]
pub fn final_card(props: &FinalCardProps) -> Html {
    //let data_state = use_store_value::<DataState>();
    let navigator = use_navigator().unwrap();

    let on_continue_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ClickAnotherReading));
            navigator.replace(&Route::Advanced {});
        })
    };

    let on_survey_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ClickSurvey));
            open_link_in_new_tab("https://docs.google.com/forms/d/e/1FAIpQLSep7npbKOtYcA_ruRFK8ByHz0Zjl_7-gp6YQ3XPhJ_QHLgw4w/viewform".to_string());
        })
    };
    let on_discord_click = {
        Callback::from(move |_e: MouseEvent| {
            Dispatch::<AchievementsState>::new()
                .apply(AchievementEarnedMessage(Achievement::ClickDiscord));
            open_link_in_same_tab("https://discord.gg/eRm5YdMNAw".to_string());
        })
    };

    let mut card_classes = classes!("prophecy-card");
    let image_classes = classes!("prophecy-image");

    if props.top_card {
        card_classes.push("top_card");
    };

    let share_text = include_str!(r#"../text/opening_p1.txt"#);
    let src_data = props.src_data.clone();

    html! {

            <div class={card_classes} style = {props.style.get_style()} >
                {
                    if props.top_card{
                        html!(<div class="prophecy-back"/>)
                    }
                    else{
                        html!(<></>)
                    }
                }

                <img class={image_classes} style="background: linear-gradient(30deg, darkgrey, lightgrey);" />

                <div class="image-overlay" style="pointer-events:none;">
                    <div class ="buttons-grid">
                        <Logo clickable={false}/>
                        <button class="nice-button card-button" style="pointer-events:auto;" onclick={on_continue_click} >{"Do another reading"} </button>
                        <ShareButton label="Share your reading" {share_text} {src_data}/>
                        <button class="nice-button card-button" style="pointer-events:auto;" onclick={on_discord_click}  >{"Discuss on Discord"} </button>
                        <button class="nice-button card-button" style="pointer-events:auto;" onclick={on_survey_click}  >{"Fill out our two minute survey"} </button>
                    </div>

                </div>

        </div>

    }
}
