use std::str::FromStr;

use crate::structs::country_city::{DenmarkCity, NorwayCity, SwedenCity};
use crate::Error;

pub trait City: Into<EveryCity> + AsRef<str> + poise::ChoiceParameter {}

#[derive(Debug, poise::ChoiceParameter)]
pub enum EveryCity {
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

impl AsRef<str> for EveryCity {
    fn as_ref(&self) -> &str {
        match self {
            EveryCity::Oslo => "Oslo, Norge",
            EveryCity::Stavanger => "Stavanger, Norge",
            EveryCity::Drammen => "Drammen, Norge",
            EveryCity::Lillehammer => "Lillehammer, Norge",
            EveryCity::Bodø => "Bodø, Norge",
            EveryCity::Stockholm => "Stockholm, Sverige",
            EveryCity::Göteborg => "Göteborg, Sverige",
            EveryCity::København => "København, Danmark",
            EveryCity::Århus => "Århus, Danmark",
        }
    }
}

impl FromStr for EveryCity {
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

impl From<NorwayCity> for EveryCity {
    fn from(value: NorwayCity) -> Self {
        match value {
            NorwayCity::Oslo => EveryCity::Oslo,
            NorwayCity::Stavanger => EveryCity::Stavanger,
            NorwayCity::Drammen => EveryCity::Drammen,
            NorwayCity::Lillehammer => EveryCity::Lillehammer,
            NorwayCity::Bodø => EveryCity::Bodø,
        }
    }
}

impl From<SwedenCity> for EveryCity {
    fn from(value: SwedenCity) -> Self {
        match value {
            SwedenCity::Stockholm => EveryCity::Stockholm,
            SwedenCity::Göteborg => EveryCity::Göteborg,
        }
    }
}

impl From<DenmarkCity> for EveryCity {
    fn from(value: DenmarkCity) -> Self {
        match value {
            DenmarkCity::København => EveryCity::København,
            DenmarkCity::Århus => EveryCity::Århus,
        }
    }
}

impl City for EveryCity {}