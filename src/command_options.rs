use std::str::FromStr;
use serde::Deserialize;

use crate::Error;

#[derive(Debug, poise::ChoiceParameter)]
pub enum City {
    Oslo,
    Stavanger,
    Drammen,
    Lillehammer,
    Bodø,
    Stockholm,
    København,
}

#[derive(Debug, Deserialize, poise::ChoiceParameter, PartialEq, Eq, Clone, Copy)]
pub enum Country {
    NO,
    SE,
    DK,
}

impl AsRef<str> for City {
    fn as_ref(&self) -> &str {
        match self {
            City::Oslo => "Oslo, Norge",
            City::Stavanger => "Stavanger, Norge",
            City::Drammen => "Drammen, Norge",
            City::Lillehammer => "Lillehammer, Norge",
            City::Bodø => "Bodø, Norge",
            City::Stockholm => "Stockholm, Sverige",
            City::København => "København, Danmark",
        }
    }
}

impl FromStr for City {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Oslo" | "oslo" => Ok(Self::Oslo),
            "Stavanger" | "stavanger" => Ok(Self::Stavanger),
            "Drammen" | "drammen" => Ok(Self::Drammen),
            "Lillehammer" | "lillehammer" => Ok(Self::Lillehammer),
            "Bodø" | "bodø" => Ok(Self::Bodø),
            x => Err(x.into())
        }
    }
}

impl From<City> for Country {
    fn from(value: City) -> Self {
        match value {
            City::Oslo | City::Stavanger | City::Drammen |
            City::Lillehammer | City::Bodø => Self::NO,
            City::Stockholm => Self::SE,
            City::København => Self::DK,
        }
    }
}