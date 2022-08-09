#![allow(unused_imports)]
mod set;
mod card;
mod game;
use set::*;
use card::*;
use serde::Deserialize;


fn main() {
    let aer_set = Set::from_code("aer").unwrap();
    println!("{:#?}", aer_set);
    // let card = Card::from_id("c896643e-eeef-49a6-a1ca-2577b55af2b0").unwrap();
    // println!("{}", card);
}
