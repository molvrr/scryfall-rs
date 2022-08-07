use serde::Deserialize;
use chrono::NaiveDate;
use crate::card::*;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Set {
    pub id: String,
    pub code: String,
    pub mtgo_code: Option<String>,
    pub tcgplayer_id: Option<usize>,
    pub name: String,
    pub set_type: String,
    #[serde(serialize_with = "to_ts")]
    pub released_at: NaiveDate,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_code: Option<String>,
    pub card_count: usize,
    pub printed_size: Option<usize>,
    pub digital: bool,
    pub foil_only: bool,
    pub nonfoil_only: bool,
    pub scryfall_uri: String,
    pub uri: String,
    pub icon_svg_uri: String,
    pub search_uri: String,
}

impl Set {
    pub fn cards(&self) -> Result<Vec<Card>, reqwest::Error>  {
        let cards_response = reqwest::blocking::get(&self.search_uri)?;
        let cards_data: Result<CardData, reqwest::Error> = cards_response.json();
        let parsed_cards_data = cards_data?;
        Ok(parsed_cards_data.data)
    }
}