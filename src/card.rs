use serde::Deserialize;

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
    pub object: String,
    pub oracle_id: String,
    pub prints_search_uri: String,
    pub rulings_uri: String,
    pub scryfall_uri: String,
    pub uri: String,
}

#[derive(Deserialize, Debug)]
pub struct CardData {
    pub data: Vec<Card>
}
