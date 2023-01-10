use std::{collections::BTreeMap, io::BufRead, str::FromStr};

use crate::data::prelude::*;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Default)]
pub struct ImageDescriptionState {
    pub descriptions: Option<BTreeMap<(Guide, Card), ImageDescription>>,
}

impl ImageDescriptionState {
    pub async fn setup() {
        // log::info!("Began Descriptions Set Up");
        let result = Self::create().await.unwrap();
        Dispatch::<ImageDescriptionState>::new().set(result);
        // log::info!("Finished Descriptions Set Up");
    }

    pub async fn create() -> Result<Self, anyhow::Error> {
        let url = "https://docs.google.com/spreadsheets/d/e/2PACX-1vRaUghYOFzr0u7ZG1y8gwYFs7GMIcWE4gbIOJ1cSQuJJReHBkkuf0MwbeOsmEtqcMBykkEhfns4n6ol/pub?gid=0&single=true&output=tsv";
        let result = reqwest::get(url).await;
        let data = result?;
        let bytes = data.bytes().await?;
        let lines = bytes.lines();
        let descriptions: BTreeMap<_, _> = lines
            .skip(1) //skip headers
            .filter_map(|x| x.ok())
            .map(move |x| ImageDescription::from_str(x.as_str()).unwrap())
            .map(|x| ((x.guide, x.card), x))
            .collect();

        Ok(ImageDescriptionState {
            descriptions: descriptions.into(),
        })
    }
}
