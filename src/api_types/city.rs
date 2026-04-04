use std::str::FromStr;


use crate::{Error, api_types::country_city::{DenmarkCity, NorwayCity, SwedenCity}};

#[derive(Debug, poise::ChoiceParameter)]
pub enum City {
    Oslo,
    Stavanger,
    Drammen,
    Lillehammer,
    Bodø,
    Stockholm,
    Göteborg,
    København,
    Århus,
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
            City::Göteborg => "Göteborg, Sverige",
            City::København => "København, Danmark",
            City::Århus => "Århus, Danmark",
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
            "Stockholm" | "stockholm" => Ok(Self::Stockholm),
            "Göteborg" | "götebrog" => Ok(Self::Göteborg),
            "København" | "kjøbenhavn" => Ok(Self::København),
            "Århus" | "århus" => Ok(Self::Århus),
            x => Err(x.into())
        }
    }
}

impl From<NorwayCity> for City {
    fn from(value: NorwayCity) -> Self {
        match value {
            NorwayCity::Oslo => City::Oslo,
            NorwayCity::Stavanger => City::Stavanger,
            NorwayCity::Drammen => City::Drammen,
            NorwayCity::Lillehammer => City::Lillehammer,
            NorwayCity::Bodø => City::Bodø,
        }
    }
}

impl From<SwedenCity> for City {
    fn from(value: SwedenCity) -> Self {
        match value {
            SwedenCity::Stockholm => City::Stockholm,
            SwedenCity::Göteborg => City::Göteborg,
        }
    }
}

impl From<DenmarkCity> for City {
    fn from(value: DenmarkCity) -> Self {
        match value {
            DenmarkCity::København => City::København,
            DenmarkCity::Århus => City::Århus,
        }
    }
}