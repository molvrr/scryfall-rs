mod set;
mod card;
mod game;
#[allow(unused_imports)]
use set::*;
use card::*;
#[allow(unused_imports)]
use serde::Deserialize;


fn main() {
    // let aer_set = Set::from_code("aer").unwrap();
    // println!("{:?}", aer_set.cards());
    let card = Card::from_id("f295b713-1d6a-43fd-910d-fb35414bf58a").unwrap();
    println!("{}", card);
}
