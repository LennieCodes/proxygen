use std::collections::BTreeMap;
use super::Card;
use super::super::error::ProxygenError;

use super::super::serde_json;

use std::iter::FromIterator;

const ALLCARDS_JSON: &'static str = include_str!("AllCards.json"); //http://mtgjson.com/json/AllCards.json.zip

// Allow non snake case for automatic deserialize
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
struct DatabaseEntry {
    name: String,
    manaCost: Option<String>,
    supertypes: Option<Vec<String>>,
    types: Option<Vec<String>>,
    subtypes: Option<Vec<String>>,
    text: Option<String>,
    power: Option<String>,
    toughness: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    map: BTreeMap<String, DatabaseEntry>,
}

fn make_database() -> Database {
    let bad_map: BTreeMap<String, DatabaseEntry> = serde_json::from_str(ALLCARDS_JSON).unwrap();

    let good_map = BTreeMap::from_iter(bad_map.iter().map(|(bad_key, value)| {
        let mut good_key = bad_key.clone();
        let good_value = value.clone();

        good_key = good_key.replace("\u{fb}", "u");

        (good_key, good_value)

    }));
    Database { map: good_map }
}

lazy_static!{
    pub static ref DATABASE: Database = make_database();
}

impl Database {
    pub fn get(&self, card_name: &str) -> Result<Card, ProxygenError> {
        let entry = match self.map.get(card_name) {
            Some(v) => v,
            None => return Err(ProxygenError::InvalidCardName(String::from(card_name))),
        };

        // TODO: Get rid of these clones, jesus christ

        Ok(Card {
            name: entry.name.clone(),
            cost: entry.manaCost.clone().unwrap_or_default(),
            typeline: {
                let mut typeline = String::new();
                for t in entry.supertypes.clone().unwrap_or_default() {
                    typeline.push_str(&t);
                    typeline.push_str(" ");
                }
                for t in entry.types.clone().unwrap_or_default() {
                    typeline.push_str(&t);
                    typeline.push_str(" ");
                }
                if entry.subtypes.is_some() {
                    typeline.push_str(" \u{2014}");
                    for t in entry.subtypes.clone().unwrap_or_default() {
                        typeline.push_str(" ");
                        typeline.push_str(&t);
                    }
                }
                typeline
            },
            text: entry.text.clone().unwrap_or_default(),
            power_toughness: {
                match entry.power.clone() {
                    Some(pow) => {
                        let tou = entry.toughness.clone().unwrap();
                        Some((pow, tou))
                    }
                    None => None,
                }
            },
        })
    }
}
