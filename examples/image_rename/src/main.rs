use std::{
    collections::{HashMap, HashSet},
    fs::DirEntry,
    str::FromStr,
};

use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use card::Card;
use guide::Guide;
use strum::IntoEnumIterator;

pub mod card;
pub mod guide;

fn main() -> Result<(), anyhow::Error> {
    let dir_path = r#"C:\Users\wainw\Pictures\EighthArcana\SecondPhaseUpscaled\jpg"#;

    let existing_image_names_path = r#"C:\Users\wainw\Pictures\EighthArcana\image_names.tsv"#;

    let mut all_image_names: HashSet<String> = std::fs::read_to_string(existing_image_names_path)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

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

        let mut iterator = vec.iter();
        let mut i = 0;
        while let Some(path) = iterator.next() {
            let p1 = path.path();

            'inner: loop {
                let new_name = format!(
                    "{g}{c}{}",
                    Engine::encode(&BASE64_URL_SAFE_NO_PAD, [i as u8])
                );
                if all_image_names.insert(new_name.clone()) {
                    let extension = p1.extension().unwrap().to_string_lossy();
                    let new_name_with_extension = format!("{new_name}.{extension}");

                    std::fs::rename(
                        path.path(),
                        format!("{dir_path}\\{new_name_with_extension}"),
                    )?;
                    break 'inner;
                } else {
                    i += 1;
                }
            }
            i += 1;
        }
    }

    let mut new_all_names: Vec<_> = all_image_names.into_iter().collect();
    new_all_names.sort();

    std::fs::write(existing_image_names_path, new_all_names.join("\n")).unwrap();

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
