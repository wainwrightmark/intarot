use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    let document_id = "2PACX-1vRaUghYOFzr0u7ZG1y8gwYFs7GMIcWE4gbIOJ1cSQuJJReHBkkuf0MwbeOsmEtqcMBykkEhfns4n6ol";
    let root_path = "..\\..\\src\\tsv\\";
    //let data_url = "https://docs.google.com/spreadsheets/d/1QhHenFO5-31uvZ3ocWLtViIrtQGxMOsfL3Xw651XYKc";
    //https://docs.google.com/spreadsheets/d/e/2PACX-1vRaUghYOFzr0u7ZG1y8gwYFs7GMIcWE4gbIOJ1cSQuJJReHBkkuf0MwbeOsmEtqcMBykkEhfns4n6ol/pub?gid=1471362191&single=true&output=tsv

    let sheet_paths = vec![

    SheetPath{
        path: root_path.to_string() + "descriptions.tsv",
        sheet_id: 0
    },SheetPath{
        path: root_path.to_string() + "spreads.tsv",
        sheet_id: 245469862
    },SheetPath{
        path: root_path.to_string() + "guide_prompts.tsv",
        sheet_id: 1471362191
    },SheetPath{
        path: root_path.to_string() + "spread_prompts.tsv",
        sheet_id: 1784466673
    },

    ];

    for sp in sheet_paths{
        let sheet_id = sp.sheet_id;
        let url = format!("https://docs.google.com/spreadsheets/d/e/{document_id}/pub?gid={sheet_id}&single=true&output=tsv");
        let contents = reqwest::get(url).await?
        .text()
        .await?;

        let path =  sp.path;
        tokio::fs::write(path, contents).await?

    }


    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SheetPath{
    pub path: String,
    pub sheet_id: usize
}