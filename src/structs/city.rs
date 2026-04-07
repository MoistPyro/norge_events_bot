use std::str::FromStr;

use crate::Error;

pub trait City: FromStr + AsRef<str> + poise::ChoiceParameter {}

#[derive(Debug, poise::ChoiceParameter)]
pub enum NorwayCity {
    Oslo,
    Stavanger,
    Lillehammer,
    Bodø,
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum SwedenCity {
    Stockholm,
    Göteborg,
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum DenmarkCity {
    København,
    Århus,
}

impl FromStr for NorwayCity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Oslo" | "oslo" => Ok(Self::Oslo),
            "Stavanger" | "stavanger" => Ok(Self::Stavanger),
            "Lillehammer" | "lillehammer" => Ok(Self::Lillehammer),
            "Bodø" | "bodø" => Ok(Self::Bodø),
            x => Err(x.into())
        }
    }
}

impl FromStr for SwedenCity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Stockholm" | "stockholm" => Ok(Self::Stockholm),
            "Göteborg" | "götebrog" => Ok(Self::Göteborg),
            x => Err(x.into())
        }
    }
}

impl FromStr for DenmarkCity {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "København" | "københavn" => Ok(Self::København),
            "Århus" | "århus" => Ok(Self::Århus),
            x => Err(x.into())
        }
    }
}

impl AsRef<str> for NorwayCity {
    fn as_ref(&self) -> &str {
        match self {
            NorwayCity::Oslo => "Oslo, Norge",
            NorwayCity::Stavanger => "Stavanger, Norge",
            NorwayCity::Lillehammer => "Lillehammer, Norge",
            NorwayCity::Bodø => "Bodø, Norge",
        }
    }
}

impl AsRef<str> for SwedenCity {
    fn as_ref(&self) -> &str {
        match self {
            SwedenCity::Stockholm => "Stockholm, Sverige",
            SwedenCity::Göteborg => "Göteborg, Sverige",
        }
    }
}

impl AsRef<str> for DenmarkCity {
    fn as_ref(&self) -> &str {
        match self {
            DenmarkCity::København => "København, Danmark",
            DenmarkCity::Århus => "Århus, Danmark",
        }
    }
}

impl City for NorwayCity {}
impl City for SwedenCity {}
impl City for DenmarkCity {}