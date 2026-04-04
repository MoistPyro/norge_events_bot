use chrono::Duration;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
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