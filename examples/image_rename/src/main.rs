use std::{collections::HashMap, fs::DirEntry, str::FromStr};

use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use card::Card;
use guide::Guide;
use strum::IntoEnumIterator;

pub mod card;
pub mod guide;

fn main() -> Result<(), anyhow::Error> {
    let dir_path = r#"C:\Users\wainw\Pictures\EighthArcana\BlankImages"#;

    let mut map: HashMap<(Guide, Card), Vec<DirEntry>> = Default::default();

    for path in std::fs::read_dir(dir_path)? {
        let path = path?;
        //let old_name = ;
        let meta = ImageMeta::from_str(path.file_name().to_str().unwrap())?;
        let key = (meta.guide, meta.card);
        map.entry(key).or_insert(Default::default()).push(path);
    }

    for ((guide, card), vec) in map {
        let g = guide.short_name();
        let c = (b'a' + (card as u8)) as char;
        for (i, path) in vec.iter().enumerate() {
            let p1 = path.path();
            let extension = p1.extension().unwrap().to_string_lossy();
            let new_name = format!(
                "{g}{c}{}.{extension}",
                Engine::encode(&BASE64_URL_SAFE_NO_PAD, [i as u8])
            );

            std::fs::rename(path.path(), format!("{dir_path}\\{new_name}"))?;

            //println!("{} ::: {new_name}", path.file_name().to_string_lossy());
        }
    }

    Ok(())
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ImageMeta {
    pub file_name: &'static str,
    pub guide: Guide,
    pub card: Card,
}

impl FromStr for ImageMeta {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file_name = Box::leak(s.to_string().into_boxed_str());

        let guide = Guide::iter()
            .find(|ss| ss.filter_image(file_name))
            .ok_or_else(|| anyhow::anyhow!("Could not find guide for {file_name}"))?;

        let card = Card::iter()
            .find(|ss| ss.filter_image(file_name))
            .ok_or_else(|| anyhow::anyhow!("Could not find card for {file_name}"))?;

        Ok(ImageMeta {
            file_name,
            guide,
            card,
        })
    }
}
