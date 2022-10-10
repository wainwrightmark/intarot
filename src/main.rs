use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::EnumIter;
use strum::EnumString;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yewdux::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container" style="display: flex; flex-direction: column;">
        <SignDropdown/>
        <SentimentDropdown />
        <ImageView />
        </div>

    }
}

#[derive(PartialEq, Eq, Store, Clone, Default, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct ImageState {
    pub sign: Option<Sign>,
    pub sentiment: Option<Sentiment>,
}

impl ImageState {
    pub fn get_image_meta(&self) -> Option<Vec<ImageMeta>> {
        if let Some(sign) = self.sign{
            if let Some(sentiment) = self.sentiment{
                let metas = IMAGEMETAS.iter().filter(|x|x.sign == sign && x.sentiment == sentiment).cloned().collect_vec();

                return Some(metas);
            }
        }

        return None;
    }
}



#[function_component(ImageView)]
pub fn image_view() -> Html {
    let (image_state, _) = use_store::<ImageState>();
    let image_meta_option = image_state.get_image_meta();
    

    if let Some(image_meta) = image_meta_option {
        let images = image_meta
            
            .iter()
            .map(|img| {
                html! {
                    <img src={format!("https://drive.google.com/uc?export=view&id={}", img.id.clone()) }/>
                }
            })
            .collect_vec();

        html! {
            <div class="grid">
  {images}
</div>
        }
    } else {
        html!(<></>)
    }
}

#[function_component(SignDropdown)]
pub fn sign_dropdown() -> Html {
    let onchange = Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        if let Ok(sign) = Sign::from_str(&value) {
            s.sign = Some(sign);
        } else {
            s.sign = None;
        }
    });

    let current_sign = use_selector::<ImageState, _, _>(|x| x.sign);

    let options = Sign::iter()
        .into_iter()
        .map(|sign| {

            let selected = Some(sign) == *current_sign;
            html!(  <option value={sign.to_string()} {selected}>{sign.to_string()}</option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange}>
        <option value="" disabled={true} selected={current_sign.is_none()}>{"Star Sign"}</option>

            {options}
        </select>
    )
}

#[function_component(SentimentDropdown)]
pub fn sentiment_dropdown() -> Html {
    let onchange = Dispatch::<ImageState>::new().reduce_mut_callback_with(|s, e: Event| {
        let input: HtmlSelectElement = e.target_unchecked_into();
        let value = input.value();
        if let Ok(sentiment) = Sentiment::from_str(&value) {
            s.sentiment = Some(sentiment);
        } else {
            s.sentiment = None;
        }
    });

    let current_sentiment = use_selector::<ImageState, _, _>(|x| x.sentiment);

    let options = Sentiment::iter()
        .into_iter()
        .map(|sentiment| {
            let selected = Some(sentiment) == *current_sentiment;
            html!(  <option value={sentiment.to_string()} {selected}>{sentiment.to_string()}</option>
            )
        })
        .collect_vec();

    html!(
        <select {onchange}>
        <option value="" disabled={true} selected={current_sentiment.is_none()}>{"Sentiment"}</option>

            {options}
        </select>
    )
}

#[derive(
    PartialEq,
    Eq,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    EnumIter,
    strum::Display,
    strum::AsRefStr,
    EnumString,
)]
pub enum Sign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

#[derive(
    PartialEq,
    Eq,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    EnumIter,
    strum::Display,
    strum::AsRefStr,
    EnumString,
)]
pub enum Sentiment {
    Joy,
    Prosperity,
    #[strum(serialize = "Good Fortune")]
    GoodFortune,
    Wealth,
    Love,
    Romance,
    Success,
    Excitement,
    Passion,
    #[strum(serialize = "Good Luck")]
    GoodLuck,
    Vitality,
    Strength,
}

#[derive(PartialEq, Eq, Clone)]
pub struct ImageMeta {
    pub id: String,
    pub sign: Sign,
    pub sentiment: Sentiment,
}

impl FromStr for ImageMeta{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, file_name) = s.split_once("\t").unwrap();

        let captures = RE.captures(file_name).unwrap();
        let sentiment = Sentiment::from_str(captures.name("sentiment").unwrap().as_str()).unwrap(); 
        let sign = Sign::from_str(captures.name("sign").unwrap().as_str()).unwrap(); 

        Ok(
            ImageMeta{
                id: id.to_string(),
                sign,
                sentiment
            }    
        )
        
    }
}

include_flate::flate!(static DATA: str from "data.txt");

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"^\d+-\d+-(?P<sentiment>.+?),\s*(?P<sign>.+?)\.jpg$").unwrap();
}

lazy_static::lazy_static! {
    pub static ref IMAGEMETAS: Vec<ImageMeta> = DATA.lines().map(|x|ImageMeta::from_str(x).unwrap()).collect_vec();
}