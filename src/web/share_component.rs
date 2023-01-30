use yew::prelude::*;
use yew_hooks::use_clipboard;
use yewdux::prelude::Dispatch;

use crate::{data::prelude::*, state::prelude::*};

#[derive(Properties, PartialEq)]
pub struct ShareProps {
    pub title: AttrValue,
    pub src_data: SrcData,
    // pub url: AttrValue,
    // pub url_has_search: bool,
    pub text: AttrValue,
    pub media: AttrValue,
}

#[function_component(ShareComponent)]
pub fn share_component(props: &ShareProps) -> Html {
    let clipboard = use_clipboard();
    let title = url_escape::encode_fragment(&props.title);
    let text = url_escape::encode_fragment(&props.text);
    //let url = url_escape::encode_www_form_urlencoded(&props.url);
    let media = url_escape::encode_www_form_urlencoded(&props.media);
    let fb_app_id: i64 = 1606026743178281;

    let fb_url = props.src_data.encoded_url_with_ref("fb");
    let pintrest_url = props.src_data.encoded_url_with_ref("pintrest");
    let twitter_url = props.src_data.encoded_url_with_ref("twitter");
    let whatsapp_url = props.src_data.encoded_url_with_ref("wa");
    let reddit_url = props.src_data.encoded_url_with_ref("reddit");
    let telegram_url = props.src_data.encoded_url_with_ref("telegram");
    let mastodon_url = props.src_data.encoded_url_with_ref("mastodon");

    //let facebook_href = format!("https://www.facebook.com/sharer/sharer.php?u={url}");
    let facebook_href = format!("
        https://www.facebook.com/dialog/share?app_id={fb_app_id}&display=page&href={fb_url}&redirect_uri={fb_url}&hashtag=#intarot"


    );
    let pinterest_href = format!(
        "https://pinterest.com/pin/create/button/?url={pintrest_url}&description={text}&media={media}"
    );
    let twitter_href = format!("https://twitter.com/intent/tweet?url={twitter_url}&text={text}"); //${t.via?`&via=${t.via}
    let whatsapp_href =
        format!("https://wa.me/?text={title}%0D%0A{whatsapp_url}%0D%0A%0D%0A{text}"); //${t.via?`&via=${t.via}
    let reddit_href = format!("https://www.reddit.com/submit?title={title}&url={reddit_url}");
    let telegram_href = format!("https://telegram.me/share/url?url={telegram_url}&text={text}");
    let mastodon_href =
        format!("https://toot.kytta.dev/?text={title}%0D%0A{mastodon_url}%0D%0A%0D%0A{text}"); // ${t.via?`%0D%0A%0D%0A${t.via}`:""}");
                                                                                               //     Facebook

    // Pintrest
    // Twitter

    // Whatsapp
    // Reddit
    // telegram
    // Mastodon

    // let url_string = url.to_string();
    //Clipboard
    let on_clipboard_click = {
        let clipboard = clipboard;
        let str = props.src_data.raw_url_with_ref("share");
        Callback::from(move |_e: MouseEvent| {
            //log::info!("Writing to Clipboard {}", str.clone());
            clipboard.write_text(str.clone());
        })
    };

    html!(
        <div class="shareon" style="display: grid; grid-template-columns: auto auto auto auto;">
            <a class="share-icon facebook" href={facebook_href} target="_blank"></a>
            <a class="share-icon pinterest" href={pinterest_href} target="_blank"></a>
            <a class="share-icon twitter" href={twitter_href} target="_blank"></a>
            <a class="share-icon whatsapp" href={whatsapp_href} target="_blank"></a>
            <a class="share-icon reddit" href={reddit_href} target="_blank"></a>
            <a class="share-icon telegram" href={telegram_href} target="_blank"></a>
            <a class="share-icon mastodon" href={mastodon_href} target="_blank"></a>
            <a class="share-icon clipboard" onclick={on_clipboard_click}  ></a>
        </div>
    )
}
