mod set;
mod card;
mod game;
#[allow(unused_imports)]
use set::*;
use card::*;
#[allow(unused_imports)]
use serde::Deserialize;


fn main() {
    let aer_set = Set::from_code("aer").unwrap();
    println!("{:#?}", aer_set.cards().unwrap());
    // let card = Card::from_id("c896643e-eeef-49a6-a1ca-2577b55af2b0").unwrap();
    // println!("{}", card);
}
