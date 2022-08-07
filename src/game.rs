use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Color {
    W,
    U,
    B,
    R,
    G,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
}

#[derive(Eq, PartialEq, Deserialize, Debug, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Standard,
    Future,
    Historic,
    Gladiator,
    Pioneer,
    Explorer,
    Modern,
    Legacy,
    Pauper,
    Vintage,
    Penny,
    Commander,
    Brawl,
    HistoricBrawl,
    Alchemy,
    PauperCommander,
    Duel,
    OldSchool,
    PreModern,
}
