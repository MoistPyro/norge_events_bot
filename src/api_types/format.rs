use std::str::FromStr;

use chrono::Duration;
use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub enum Format {
    #[serde(alias = "Sealed Deck")]
    #[serde(alias = "sealed deck")]
    SealedDeck(String),
    #[serde(alias = "Booster Draft")]
    #[serde(alias = "booster draft")]
    BoosterDraft(String),
    #[serde(alias = "Blitz Preconstructed")]
    #[serde(alias = "blitz preconstructed")]
    BlitzPrecon(String),
    Blitz(String),
    #[serde(alias = "Classic Constructed")]
    #[serde(alias = "classic constructed")]
    ClassicConstructed(String),
    #[serde(alias = "Living Legend")]
    #[serde(alias = "living legend")]
    LivingLegend(String),
    Commoner(String),
    #[serde(alias = "Ultimate Pit Fight")]
    #[serde(alias = "ultimate pit fight")]
    UltimatePitFight(String),
    #[serde(alias = "Crack, Shuffle, Play!")]
    #[serde(alias = "crack, shuffle, play!")]
    CrackShufflePlay(String),
    #[serde(alias = "Silver Age")]
    #[serde(alias = "silver age")]
    Sage(String),
}

impl AsRef<str> for Format {
    fn as_ref(&self) -> &str {
        match self {
            Format::SealedDeck(_) => "Sealed Deck",
            Format::BoosterDraft(_) => "Booster Draft",
            Format::BlitzPrecon(_) => "Blitz Preconstructed",
            Format::Blitz(_) => "Blitz",
            Format::ClassicConstructed(_) => "Classic Constructed",
            Format::LivingLegend(_) => "Living Legend",
            Format::Commoner(_) => "Commoner",
            Format::UltimatePitFight(_) => "Ultimate Pit Fight",
            Format::CrackShufflePlay(_) => "Crack, Shuffle, Play!",
            Format::Sage(_) => "Silver Age",
        }
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sealed deck" => Ok(Self::SealedDeck("!".to_string())),
            "booster draft" => Ok(Self::BoosterDraft("!".to_string())),
            "blitz preconstructed" => Ok(Self::BlitzPrecon("?".to_string())),
            "blitz" => Ok(Self::Blitz("!".to_string())),
            "classic constructed" => Ok(Self::ClassicConstructed("!".to_string())),
            "living legend" => Ok(Self::LivingLegend("!".to_string())),
            "commoner" => Ok(Self::Commoner("!".to_string())),
            "ultimate pit fight" => Ok(Self::UltimatePitFight("!".to_string())),
            "crack, shuffle, play!" => Ok(Self::CrackShufflePlay("!".to_string())),
            "silver age" => Ok(Self::Sage("!".to_string())),
            _ => Err("Unknown format".into()),
        }
    }
}

impl Format {
    pub fn duration(&self) -> Duration {
        match self {
            Format::SealedDeck(_) |
            Format::BoosterDraft(_) |
            Format::BlitzPrecon(_) |
            Format::Blitz(_) |
            Format::Commoner(_) |
            Format::CrackShufflePlay(_) |
            Format::Sage(_) => Duration::minutes(40),
            Format::ClassicConstructed(_) |
            Format::LivingLegend(_) => Duration::minutes(60),
            _ => Duration::zero()
        }
    }
}