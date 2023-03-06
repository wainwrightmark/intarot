#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r##"export function get_referrer() {
  return document.referrer;
}"##)]
extern "C" {
    pub fn get_referrer() -> String;
}


// #[wasm_bindgen::prelude::wasm_bindgen(inline_js = r##"export function get_referrer() {
//   return document.referrer;
// }"##)]
// extern "C" {
//     pub fn get_referrer() -> String;
// }



#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r##"export function open_link_in_new_tab(url) {
    window.open(url, '_blank').focus();
}"##)]
extern "C" {
    pub fn open_link_in_new_tab(url: String);
}

#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r##"export function open_link_in_same_tab(url) {
    window.open(url).focus();
}"##)]
extern "C" {
    pub fn open_link_in_same_tab(url: String);
}

#[wasm_bindgen::prelude::wasm_bindgen(inline_js = r##"export function scroll_to_top() {
    window.scrollTo(0,0);
}"##)]
extern "C" {
    pub fn scroll_to_top();
}

#[wasm_bindgen::prelude::wasm_bindgen(
    inline_js = r##"export function angry_animate_top_card_right() {

    document.querySelector(".top_card").animate(
    [
        { transform: 'translateX(0) rotate(0deg)' },
        { transform: 'translateX(100px) rotate(-10deg)' },
        { transform: 'translateX(50px) rotate(10deg)' },
        { transform: 'translateX(70px) rotate(-15deg)' },
        { transform: 'translateX(50px) rotate(5deg)' },
        { transform: 'translateX(0) rotate(0deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }"##
)]
extern "C" {
    pub fn angry_animate_top_card_right();
}

#[wasm_bindgen::prelude::wasm_bindgen(
    inline_js = r##"export function angry_animate_top_card_left() {

    document.querySelector(".top_card").animate(
    [
      { transform: 'translateX(0) rotate(0deg)' },
      { transform: 'translateX(-100px) rotate(10deg)' },
      { transform: 'translateX(-50px) rotate(-10deg)' },
      { transform: 'translateX(-70px) rotate(15deg)' },
      { transform: 'translateX(-50px) rotate(-5deg)' },
      { transform: 'translateX(0) rotate(0deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }"##
)]
extern "C" {
    pub fn angry_animate_top_card_left();
}

#[wasm_bindgen::prelude::wasm_bindgen(
    inline_js = r##"export function angry_animate_top_card_right_facedown() {

    document.querySelector(".top_card").animate(
    [
        { transform: 'translateX(0) rotate(0deg) rotateY(180deg)' },
        { transform: 'translateX(100px) rotate(-10deg) rotateY(180deg)' },
        { transform: 'translateX(50px) rotate(10deg) rotateY(180deg)' },
        { transform: 'translateX(70px) rotate(-15deg) rotateY(180deg)' },
        { transform: 'translateX(50px) rotate(5deg) rotateY(180deg)' },
        { transform: 'translateX(0) rotate(0deg) rotateY(180deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }"##
)]
extern "C" {
    pub fn angry_animate_top_card_right_facedown();
}

#[wasm_bindgen::prelude::wasm_bindgen(
    inline_js = r##"export function angry_animate_top_card_left_facedown() {

    document.querySelector(".top_card").animate(
    [
      { transform: 'translateX(0) rotate(0deg) rotateY(180deg)' },
      { transform: 'translateX(-100px) rotate(10deg) rotateY(180deg)' },
      { transform: 'translateX(-50px) rotate(-10deg) rotateY(180deg)' },
      { transform: 'translateX(-70px) rotate(15deg) rotateY(180deg)' },
      { transform: 'translateX(-50px) rotate(-5deg) rotateY(180deg)' },
      { transform: 'translateX(0) rotate(0deg) rotateY(180deg)' },
    ], {
      duration: 500,
      iterations: 1
    }
  ) }"##
)]
extern "C" {
    pub fn angry_animate_top_card_left_facedown();
}
