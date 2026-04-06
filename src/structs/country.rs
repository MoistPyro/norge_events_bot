use serde::Deserialize;
use super::EveryCity;

#[derive(Debug, Deserialize, poise::ChoiceParameter, PartialEq, Eq, Clone, Copy)]
pub enum Country {
    NO,
    SE,
    DK,
}

impl From<EveryCity> for Country {
    fn from(value: EveryCity) -> Self {
        match value {
            EveryCity::Oslo |
            EveryCity::Stavanger |
            EveryCity::Drammen |
            EveryCity::Lillehammer |
            EveryCity::Bodø => Self::NO,
            EveryCity::Stockholm |
            EveryCity::Göteborg => Self::SE,
            EveryCity::København |
            EveryCity::Århus => Self::DK,
        }
    }
}