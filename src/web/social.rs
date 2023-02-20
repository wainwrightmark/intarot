use yew::prelude::*;

use crate::{data::achievement::SocialPlatform, state::prelude::*};

#[derive(Properties, PartialEq)]
pub struct SocialButtonProps {}

#[function_component(SocialIcons)]
pub fn social_icons(_props: &SocialButtonProps) -> Html {
    //icons from https://www.iconfinder.com/iconsets/popular-services-brands-vol-2

    let on_discord_click = {
        Callback::from(move |_e: MouseEvent| {
            LoggableEvent::try_log(SocialPlatform::Discord);
        })
    };
    let on_facebook_click = {
        Callback::from(move |_e: MouseEvent| {
            LoggableEvent::try_log(SocialPlatform::Facebook);
        })
    };
    let on_twitter_click = {
        Callback::from(move |_e: MouseEvent| {
            LoggableEvent::try_log(SocialPlatform::Twitter);
        })
    };

    let on_instagram_click = {
        Callback::from(move |_e: MouseEvent| {
            LoggableEvent::try_log(SocialPlatform::Instagram);
        })
    };

    let on_tiktok_click = {
        Callback::from(move |_e: MouseEvent| {
            LoggableEvent::try_log(SocialPlatform::Tiktok);
        })
    };

    html! {
        <>
        <div class="social-row">

        <a href="https://www.facebook.com/profile.php?id=100089979217774" class="social-icon si-facebook tooltip" target="_blank" onclick={on_facebook_click}>
                <img src="social/facebook.svg" width="40" height="40" alt="facebook" />
                <span class="tooltiptext">{"Follow us on Facebook"}</span>
        </a>
        <a href="https://twitter.com/intarotapp" class="social-icon si-twitter tooltip" target="_blank" onclick={on_twitter_click}>
            <img src="social/twitter.svg" width="40" height="40" alt="twitter"/>
            <span class="tooltiptext">{"Follow us on Twitter"}</span>
        </a>
        <a href="https://discord.gg/eRm5YdMNAw" class="social-icon si-discord tooltip" target="_blank" onclick={on_discord_click}>
            <img src="social/discord.svg" width="40" height="40" alt="discord"/>
            <span class="tooltiptext">{"Join our Discord"}</span>
        </a>
        <a href="https://www.instagram.com/intarotapp/" class="social-icon si-instagram tooltip" target="_blank" onclick={on_instagram_click}>
            <img src="social/instagram.svg" width="40" height="40" alt="instagram"/>
            <span class="tooltiptext">{"Follow us on Instagram"}</span>
        </a>

        <a href="https://www.tiktok.com/@intarot?_t=8ZZXQSKNQIv&_r=1" class="social-icon si-tiktok tooltip" target="_blank" onclick={on_tiktok_click}>
            <img src="social/tiktok.svg" width="40" height="40" alt="tiktok"/>
            <span class="tooltiptext">{"Follow us on the clock app"}</span>
        </a>
        </div>
        </>
    }
}
