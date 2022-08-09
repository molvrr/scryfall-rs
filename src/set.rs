#![allow(dead_code)]
use serde::Deserialize;
use chrono::NaiveDate;
use crate::card::*;

#[derive(Debug, Deserialize)]
pub struct Set {
    pub id: String,
    pub code: String,
    pub mtgo_code: Option<String>,
    pub tcgplayer_id: Option<usize>,
    pub name: String,
    pub set_type: String,
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
    pub fn from_code(code: &str) -> Result<Self, reqwest::Error> {
        let response = reqwest::blocking::get(String::from("https://api.scryfall.com/sets/") + code).unwrap();
        let raw_data: Result<Set, reqwest::Error> = response.json();
        raw_data
    }

    pub fn cards(&self) -> Result<Vec<Card>, reqwest::Error>  {
        let cards_response = reqwest::blocking::get(&self.search_uri)?;
        let cards_data: Result<CardData, reqwest::Error> = cards_response.json();
        let parsed_cards_data = cards_data?;
        Ok(parsed_cards_data.data)
    }
}
