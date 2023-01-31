use yew::{function_component, html, Html, Properties, use_state};
use yewdux::prelude::{use_selector, use_store};

use crate::state::mailchimp_state::MailchimpState;

#[derive(Properties, Debug, PartialEq, Clone, Copy)]
pub struct MailchimpProps {}

#[function_component(Mailchimp)]
pub fn mailchimp(_props: &MailchimpProps) -> Html {
    let (state, dispatch) = use_store::<MailchimpState>();
    let show = state.show;

    let onchange = dispatch.reduce_callback(|_|{
        MailchimpState{
            show: false,
            has_been_bugged: true
        }.into()
    });

    html!(
    <>
    <input class="modal-state" id="modal-mailchimp" type="checkbox" checked={show} {onchange}/>
    <div class="modal">
      <label class="modal-bg" for="modal-mailchimp"></label>
      <div class="modal-body">
      <iframe src="mailchimp.html" title="Mailchimp Signup" style="height: 200px"></iframe>
      </div>
    </div>
    </>
        )
}

/*
<link href="//cdn-images.mailchimp.com/embedcode/classic-071822.css" rel="stylesheet" type="text/css" />
        <style type="text/css">
            {"#mc_embed_signup{background:#fff; clear:left; font:14px Helvetica,Arial,sans-serif;  width:600px;}"}
        </style>
        <div id="mc_embed_signup">
            <form action="https://app.us21.list-manage.com/subscribe/post?u=b9e9d767448b6f3485a30c05b&amp;id=6234ad4091&amp;f_id=00d5dee1f0" method="post" id="mc-embedded-subscribe-form" name="mc-embedded-subscribe-form" class="validate" target="_blank" novalidate=true>
                <div id="mc_embed_signup_scroll">
                <h2>{"Subscribe"}</h2>
                <div class="indicates-required"><span class="asterisk">{"*"}</span> {"indicates required"}</div>
        <div class="mc-field-group">
            <label for="mce-EMAIL">{"Email Address"} <span class="asterisk">{"*"}</span>
        </label>
            <input type="email" value="" name="EMAIL" class="required email" id="mce-EMAIL" required=true />
            <span id="mce-EMAIL-HELPERTEXT" class="helper_text"></span>
        </div>
            <div id="mce-responses" class="clear foot">
                <div class="response" id="mce-error-response" style="display:none"></div>
                <div class="response" id="mce-success-response" style="display:none"></div>
            </div>
            <div style="position: absolute; left: -5000px;" aria-hidden="true"><input type="text" name="b_b9e9d767448b6f3485a30c05b_6234ad4091" tabindex="-1" value="" /></div>
                <div class="optionalParent">
                    <div class="clear foot">
                        <input type="submit" value="Subscribe" name="subscribe" id="mc-embedded-subscribe" class="button" />
                        <p class="brandingLogo"><a href="http://eepurl.com/ijFKfv" title="Mailchimp - email marketing made easy and fun"><img src="https://eep.io/mc-cdn-images/template_images/branding_logo_text_dark_dtp.svg" /></a></p>
                    </div>
                </div>
            </div>
        </form>
        </div>
        <script type="text/javascript" src=r##"//s3.amazonaws.com/downloads.mailchimp.com/js/mc-validate.js"##></script><script type="text/javascript">
        {"        (function($) {window.fnames = new Array(); window.ftypes = new Array();fnames[0]='EMAIL';ftypes[0]='email';fnames[1]='FNAME';ftypes[1]='text';fnames[2]='LNAME';ftypes[2]='text';fnames[3]='ADDRESS';ftypes[3]='address';fnames[4]='PHONE';ftypes[4]='phone';fnames[5]='BIRTHDAY';ftypes[5]='birthday';}(jQuery));var $mcj = jQuery.noConflict(true);"}
        </script>
 */