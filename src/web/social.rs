use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SocialButtonProps {
}

#[function_component(SocialIcons)]
pub fn social_icons(_props: &SocialButtonProps) -> Html {
    //icons from https://www.iconfinder.com/iconsets/popular-services-brands-vol-2


    html!{
        <div class="social-row">
        <a href="https://www.facebook.com/profile.php?id=100089979217774" class="social-icon si-facebook tooltip" target="_blank" >
                <img src="social/facebook.svg" width="40" height="40" alt="facebook" />
                <span class="tooltiptext">{"Follow us on Facebook"}</span>
        </a>
        <a href="https://twitter.com/intarotapp" class="social-icon si-twitter tooltip" target="_blank">
            <img src="social/twitter.svg" width="40" height="40" alt="twitter"/>
            <span class="tooltiptext">{"Follow us on Twitter"}</span>
        </a>
        <a href="https://discord.gg/eRm5YdMNAw" class="social-icon si-discord tooltip" target="_blank">
            <img src="social/discord.svg" width="40" height="40" alt="discord"/>
            <span class="tooltiptext">{"Join our Discord"}</span>
        </a>
        <a href="https://www.instagram.com/intarotapp/" class="social-icon si-instagram tooltip" target="_blank">
            <img src="social/instagram.svg" width="40" height="40" alt="instagram"/>
            <span class="tooltiptext">{"Follow us on Instagram"}</span>
        </a>
        </div>
    }
}

