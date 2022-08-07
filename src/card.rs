use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "layout")]
pub enum Layout {
    Normal,
    Split {
        card_faces: Vec<CardFace>
    },
    Flip,
    Transform,
    ModalDFC,
    Meld,
    Leveler,
    Class,
    Saga,
    Adventure,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
    ArtSeries,
    ReversibleCard,
}

#[derive(Deserialize, Debug)]
pub struct CardFace {
    pub object: String,
    pub name: String,
    pub flavor_name: Option<String>,
    pub mana_cost: String,
    pub type_line: String,
    pub oracle_text: String,
    pub artist: String,
    pub artist_id: String,
    pub illustration_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum Color {
    W,
    U,
    B,
    R,
    G
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Part {
    id: String,
    component: String, // Change to enum,
    name: String,
    type_line: String,
    uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Card {
    pub arena_id: Option<usize>,
    pub id: String,
    pub lang: String,
    pub mtgo_id: Option<usize>,
    pub mtgo_foil_id: Option<usize>,
    pub multiverse_ids: Option<Vec<usize>>,
    pub tcgplayer_id: Option<usize>,
    pub tcgplayer_etched_id: Option<usize>,
    pub cardmarket_id: Option<usize>,
    // pub object: String,
    pub oracle_id: String,
    pub prints_search_uri: String,
    pub rulings_uri: String,
    pub scryfall_uri: String,
    pub uri: String,
    #[serde(flatten)]
    pub layout: Layout,
    pub highres_image: bool,
    pub cmc: f32,
    pub color_identity: Vec<Color>,
    pub all_parts: Option<Vec<Part>>,

}

#[derive(Deserialize, Debug)]
pub struct CardData {
    pub data: Vec<Card>
}

impl Card {
    pub fn from_id(id: &str) -> Result<Self, reqwest::Error> {
        let response = reqwest::blocking::get(String::from("https://api.scryfall.com/cards/") + id).unwrap();
        let raw_data: Result<Card, reqwest::Error> = response.json();
        raw_data
    }
}
