#![allow(dead_code)]
use crate::game::*;
use std::collections::hash_map::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "layout")]
pub enum Layout {
    Normal,
    Split,
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
pub struct Part {
    id: String,
    component: String, // Change to enum,
    name: String,
    type_line: String,
    uri: String,
}

#[derive(Deserialize, Debug)]
pub struct Card {
    pub id: String,
    pub lang: String,
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
    pub keywords: Vec<String>,
    pub mtgo_id: Option<usize>,
    pub mtgo_foil_id: Option<usize>,
    pub multiverse_ids: Option<Vec<usize>>,
    pub tcgplayer_id: Option<usize>,
    pub tcgplayer_etched_id: Option<usize>,
    pub cardmarket_id: Option<usize>,
    pub arena_id: Option<usize>,
    pub all_parts: Option<Vec<Part>>,
    pub card_faces: Option<Vec<CardFace>>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Option<Vec<Color>>,
    pub edhrec_rank: Option<usize>,
    pub hand_modifier: Option<String>,
    pub legalities: HashMap<Format, Legality>,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>
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

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        str.push_str(&format!("id: {}\n", self.id));
        str.push_str(&format!("name: {}\n", self.name));
        str.push_str(&format!("lang: {}\n", self.lang));
        // str.push_str(&format!("object: {}\n", self.object));
        // str.push_str(&format!("oracle_id: {}\n", self.oracle_id));
        // str.push_str(&format!("prints_search_uri: {}\n", self.prints_search_uri));
        // str.push_str(&format!("rulings_uri: {}\n", self.rulings_uri));
        // str.push_str(&format!("scryfall_uri: {}\n", self.scryfall_uri));
        // str.push_str(&format!("uri: {}\n", self.uri));
        str.push_str(&format!("mana_cost: {:?}\n", self.mana_cost.as_ref().unwrap()));
        str.push_str(&format!("cmc: {}\n", self.cmc));
        str.push_str(&format!("layout: {}\n", self.layout));
        str.push_str(&format!("highres_image: {}\n", self.highres_image));
        str.push_str(&format!("color_identity: {:?}\n", self.color_identity));
        str.push_str(&format!("keywords: {:?}\n", self.keywords));
        // str.push_str(&format!("mtgo_id: {}\n", self.mtgo_id));
        // str.push_str(&format!("mtgo_foil_id: {}\n", self.mtgo_foil_id));
        // str.push_str(&format!("multiverse_ids: {}\n", self.multiverse_ids));
        // str.push_str(&format!("tcgplayer_id: {}\n", self.tcgplayer_id));
        // str.push_str(&format!("tcgplayer_etched_id: {}\n", self.tcgplayer_etched_id));
        // str.push_str(&format!("cardmarket_id: {}\n", self.cardmarket_id));
        // str.push_str(&format!("arena_id: {}\n", self.arena_id));
        // str.push_str(&format!("all_parts: {}\n", self.all_parts));
        // str.push_str(&format!("card_faces: {}\n", self.card_faces));
        // str.push_str(&format!("color_indicator: {}\n", self.color_indicator));
        // str.push_str(&format!("colors: {}\n", self.colors));
        // str.push_str(&format!("edhrec_rank: {}\n", self.edhrec_rank));
        // str.push_str(&format!("hand_modifier: {}\n", self.hand_modifier));
        // str.push_str(&format!("legalities: {}\n", self.legalities));
        // str.push_str(&format!("life_modifier: {}\n", self.life_modifier));
        str.push_str(&format!("loyalty: {:?}\n", self.loyalty));
        str.push_str(&format!("oracle_text: {:?}", self.oracle_text.as_ref().unwrap()));
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal"),
            Self::Split => write!(f, "split"),
            Self::Flip => write!(f, "flip"),
            Self::Transform => write!(f, "transform"),
            Self::ModalDFC => write!(f, "modalDFC"),
            Self::Meld => write!(f, "meld"),
            Self::Leveler => write!(f, "leveler"),
            Self::Class => write!(f, "class"),
            Self::Saga => write!(f, "saga"),
            Self::Adventure => write!(f, "adventure"),
            Self::Planar => write!(f, "planar"),
            Self::Scheme => write!(f, "scheme"),
            Self::Vanguard => write!(f, "vanguard"),
            Self::Token => write!(f, "token"),
            Self::DoubleFacedToken => write!(f, "doubleFacedToken"),
            Self::Emblem => write!(f, "emblem"),
            Self::Augment => write!(f, "augment"),
            Self::Host => write!(f, "host"),
            Self::ArtSeries => write!(f, "artSeries"),
            Self::ReversibleCard => write!(f, "reversibleCard"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn display() {
        assert_eq!(format!("{}", Layout::Normal), "normal");
        assert_eq!(format!("{}", Layout::Split), "split");
        assert_eq!(format!("{}", Layout::Flip), "flip");
        assert_eq!(format!("{}", Layout::Transform), "transform");
        assert_eq!(format!("{}", Layout::ModalDFC), "modalDFC");
        assert_eq!(format!("{}", Layout::Meld), "meld");
        assert_eq!(format!("{}", Layout::Leveler), "leveler");
        assert_eq!(format!("{}", Layout::Class), "class");
        assert_eq!(format!("{}", Layout::Saga), "saga");
        assert_eq!(format!("{}", Layout::Adventure), "adventure");
        assert_eq!(format!("{}", Layout::Planar), "planar");
        assert_eq!(format!("{}", Layout::Scheme), "scheme");
        assert_eq!(format!("{}", Layout::Vanguard), "vanguard");
        assert_eq!(format!("{}", Layout::Token), "token");
        assert_eq!(format!("{}", Layout::DoubleFacedToken), "doubleFacedToken");
        assert_eq!(format!("{}", Layout::Emblem), "emblem");
        assert_eq!(format!("{}", Layout::Augment), "augment");
        assert_eq!(format!("{}", Layout::Host), "host");
        assert_eq!(format!("{}", Layout::ArtSeries), "artSeries");
        assert_eq!(format!("{}", Layout::ReversibleCard), "reversibleCard");
    }
}
