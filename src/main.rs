mod set;
mod card;
use set::*;
use card::*;
use serde::Deserialize;


fn main() -> Result<(), reqwest::Error> {
    let response = reqwest::blocking::get("https://api.scryfall.com/sets/aer")?;
    let raw_data: Result<Set, reqwest::Error> = response.json();
    let parsed_data = raw_data?;

    println!("{:?}", parsed_data.cards());
    Ok(())
}
