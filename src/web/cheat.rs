use crate::{
    data::prelude::{Card, Perm},
    state::prelude::*,
    web::prelude::*,
};
use anyhow::{anyhow, bail};
use itertools::Itertools;
use num_traits::FromPrimitive;
use strum::{EnumCount, IntoEnumIterator};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use yewdux::prelude::Dispatch;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct CheatProps {
    pub cards: String,
}

#[function_component(CheatView)]
pub fn cheat_view(props: &CheatProps) -> Html {
    log::info!("Cheat View");
    let navigator = use_navigator().unwrap();
    let event = LoggableEvent::Cheat {
        cards: props.cards.clone(),
    };
    LoggableEvent::try_log(event);

    match get_cheat_perm(&props.cards) {
        Ok(permutation) => {
            Dispatch::<DataState>::new().apply(SetPermutationMessage { permutation });
            navigator.push(&Route::Spread);
        }
        Err(err) => {
            LoggableEvent::try_log_error(err);
            navigator.push(&Route::Landing);
        }
    }

    html!(<Logo clickable={true} invertible={true}/>)
}

pub fn get_cheat_perm(s: &str) -> Result<Perm, anyhow::Error> {
    let cards = s
        .split_terminator('.')
        .map(|x| {
            x.parse::<u8>()
                .map_err(|x| x.into())
                .and_then(|x| Card::from_u8(x).ok_or(anyhow!("Out of range")))
        })
        .fold_ok(Vec::<Card>::new(), |mut vec, value| {
            vec.push(value);
            vec
        })?;

    if cards.len() > Card::COUNT {
        bail!("Too many cards")
    }
    if !cards.iter().all_unique() {
        bail!("Cards not unique")
    }

    let extra_cards = Card::iter().filter(|x| !cards.contains(x));

    let cards2 = cards.iter().cloned().chain(extra_cards).collect_vec();

    let perm = Perm::calculate_incomplete(cards2.as_slice());
    Ok(perm)
}
