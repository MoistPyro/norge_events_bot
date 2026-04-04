use serde::Deserialize;
use super::City;

#[derive(Debug, Deserialize, poise::ChoiceParameter, PartialEq, Eq, Clone, Copy)]
pub enum Country {
    NO,
    SE,
    DK,
}

impl From<City> for Country {
    fn from(value: City) -> Self {
        match value {
            City::Oslo |
            City::Stavanger |
            City::Drammen |
            City::Lillehammer |
            City::Bodø => Self::NO,
            City::Stockholm |
            City::Göteborg => Self::SE,
            City::København |
            City::Århus => Self::DK,
        }
    }
}