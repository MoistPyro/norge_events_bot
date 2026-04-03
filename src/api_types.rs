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
    Göteborg,
    København,
    Århus,
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum NorwayCity {
    Oslo,
    Stavanger,
    Drammen,
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

#[derive(Debug, Deserialize, poise::ChoiceParameter, PartialEq, Eq, Clone, Copy)]
pub enum Country {
    NO,
    SE,
    DK,
}

#[derive(Debug, Deserialize)]
pub enum Format {
    #[serde(rename = "Sealed Deck")]
    SealedDeck,
    #[serde(rename = "Booster Draft")]
    BoosterDraft,
    #[serde(rename = "Blitz Preconstructed")]
    BlitzPrecon,
    Blitz,
    #[serde(rename = "Classic Constructed")]
    ClassicConstructed,
    #[serde(rename = "Living Legend")]
    LivingLegend,
    Commoner,
    #[serde(rename = "Ultimate Pit Fight")]
    UltimatePitFight,
    #[serde(rename = "Crack, Shuffle, Play!")]
    CrackShufflePlay,
    #[serde(rename = "Silver Age")]
    Sage,
    #[serde(other)]
    Unknown
}

#[derive(Debug, Deserialize)]
pub enum EventType {
    #[serde(rename = "Armory Event")]
    ArmoryEvent,
    Battlegrounds,
    Calling,
    #[serde(rename = "Learn to Play Event")]
    LearnToPlay,
    #[serde(rename = "National Championship")]
    NationalChampionship,
    #[serde(rename = "On Demand")]
    OnDemand,
    #[serde(rename = "Pro Tour")]
    ProTour,
    Showdown,
    #[serde(rename = "Social Play Event")]
    SocialPlayEvent,
    #[serde(rename = "World Championship Qualifier")]
    WorldChampionshipQualifier,
    #[serde(rename = "World Premiere")]
    WorldPremiere,
    #[serde(other)]
    Unknown
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

impl AsRef<str> for NorwayCity {
    fn as_ref(&self) -> &str {
        match self {
            NorwayCity::Oslo => "Oslo, Norge",
            NorwayCity::Stavanger => "Stavanger, Norge",
            NorwayCity::Drammen => "Drammen, Norge",
            NorwayCity::Lillehammer => "Lillehammer, Norge",
            NorwayCity::Bodø => "Bodø, Norge",
        }
    }
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

impl AsRef<str> for Format {
    fn as_ref(&self) -> &str {
        match self {
            Format::SealedDeck => "Sealed Deck",
            Format::BoosterDraft => "Booster Draft",
            Format::BlitzPrecon => "Blitz Preconstructed",
            Format::Blitz => "Blitz",
            Format::ClassicConstructed => "Classic Constructed",
            Format::LivingLegend => "Living Legend",
            Format::Commoner => "Commoner",
            Format::UltimatePitFight => "Ultimate Pit Fight",
            Format::CrackShufflePlay => "Crack, Shuffle, Play!",
            Format::Sage => "Silver Age",
            Format::Unknown => "Unknown Format",
        }
    }
}

impl AsRef<str> for EventType {
    fn as_ref(&self) -> &str {
        match self {
            EventType::ArmoryEvent => "Armory Event",
            EventType::Battlegrounds => "Battlegrounds",
            EventType::Calling => "Calling",
            EventType::LearnToPlay => "Learn to Play Event",
            EventType::NationalChampionship => "National Championship",
            EventType::OnDemand => "On Demand",
            EventType::ProTour => "Pro Tour",
            EventType::Showdown => "Showdown",
            EventType::SocialPlayEvent => "Social Play Event",
            EventType::WorldChampionshipQualifier => "World Championship Qualifier",
            EventType::WorldPremiere => "World Premiere",
            EventType::Unknown => "Unknown Format",
        }
    }
}