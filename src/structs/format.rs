use std::{fmt::Display, str::FromStr};

use chrono::Duration;
use serde::Deserialize;

use crate::Error;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Format {  
    SealedDeck,
    BoosterDraft,
    BlitzPrecon,
    Blitz,
    ClassicConstructed,
    LivingLegend,
    Commoner,
    UltimatePitFight,
    CrackShufflePlay,
    Sage,
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
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "sealed deck" => Ok(Self::SealedDeck),
            "booster draft" => Ok(Self::BoosterDraft),
            "blitz preconstructed" => Ok(Self::BlitzPrecon),
            "blitz" => Ok(Self::Blitz),
            "classic constructed" => Ok(Self::ClassicConstructed),
            "living legend" => Ok(Self::LivingLegend),
            "commoner" => Ok(Self::Commoner),
            "ultimate pit fight" => Ok(Self::UltimatePitFight),
            "crack, shuffle, play!" => Ok(Self::CrackShufflePlay),
            "silver age" => Ok(Self::Sage),
            x => Err(format!("unknown format {x}").into()),
        }
    }
}

impl Format {
    pub fn duration(&self) -> Duration {
        match self {
            Format::SealedDeck |
            Format::BoosterDraft |
            Format::BlitzPrecon |
            Format::Blitz |
            Format::Commoner |
            Format::CrackShufflePlay |
            Format::Sage => Duration::minutes(40),
            Format::ClassicConstructed |
            Format::LivingLegend => Duration::minutes(60),
            _ => Duration::zero()
        }
    }
}