use std::str::FromStr;

use chrono::Duration;
use crate::Error;

use super::Format;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    ArmoryEvent,
    Battlegrounds,
    Calling,
    LearnToPlay,
    NationalChampionship,
    PreRelease,
    OnDemand,
    ProTour,
    Showdown,
    Skirmish,
    SocialPlayEvent,
    WorldChampionshipQualifier,
    WorldPremiere,
}

impl AsRef<str> for EventType {
    fn as_ref(&self) -> &str {
        match self {
            EventType::ArmoryEvent => "Armory Event",
            EventType::Battlegrounds => "Battlegrounds",
            EventType::Calling => "Calling",
            EventType::LearnToPlay => "Learn to Play Event",
            EventType::NationalChampionship => "National Championship",
            EventType::PreRelease => "Omens of the Third Age Pre-Release",
            EventType::OnDemand => "On Demand",
            EventType::ProTour => "Pro Tour",
            EventType::Showdown => "Showdown",
            EventType::Skirmish => "Skirmish Season 14",
            EventType::SocialPlayEvent => "Social Play Event",
            EventType::WorldChampionshipQualifier => "World Championship Qualifier",
            EventType::WorldPremiere => "World Premiere",
        }
    }
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "armory event" => Ok(Self::ArmoryEvent),
            "battlegrounds" => Ok(Self::Battlegrounds),
            "calling" => Ok(Self::Calling),
            "learn to play" => Ok(Self::LearnToPlay),
            "national championship" => Ok(Self::NationalChampionship),
            "omens of the third age pre-release" => Ok(Self::PreRelease),
            "on demand" => Ok(Self::OnDemand),
            "pro tour" => Ok(Self::ProTour),
            "showdown" => Ok(Self::Showdown),
            "skirmish season 14" => Ok(Self::Skirmish),
            "social play event" => Ok(Self::SocialPlayEvent),
            "world championship qualifier" => Ok(Self::WorldChampionshipQualifier),
            "world premiere" => Ok(Self::WorldPremiere),
            x => Err(format!("unknown event type {x}").into())
        }
    }
}

impl EventType {
    ///returns None if event type is unknown
    fn rounds(&self, player_cap: Option<i32>) -> Option<i32> {

        let cap_based = player_cap
            .map(|c| (c as f32).log2().ceil() as i32)
            .unwrap_or(0);

        match self {
            EventType::ArmoryEvent |
            EventType::LearnToPlay |
            EventType::OnDemand |
            EventType::SocialPlayEvent => Some(if cap_based > 0 {cap_based} else {3}),
            Self::PreRelease => Some(if cap_based > 0 {cap_based} else {4}),
            EventType::Battlegrounds |
            EventType::Showdown |
            EventType::Skirmish |
            EventType::NationalChampionship |
            EventType::WorldPremiere |
            EventType::WorldChampionshipQualifier => Some(if cap_based > 0 {cap_based} else {7}),
            EventType::Calling => Some(12),
            EventType::ProTour => Some(14),
        }
    }

    /// returns None if event type is Unknown or if overflow (this will never happen)
    pub fn duration(&self, format: &Format, player_cap: Option<i32>) -> Option<Duration> {

        self.rounds(player_cap).map(|r| {
            match self {
                Self::ProTour => format
                    .duration()
                    .checked_mul(6)
                    .zip(
                        Format::ClassicConstructed
                        .duration()
                        .checked_mul(8))
                        .map(|(a, b)| a + b),
                _ => format.duration().checked_mul(r),
            }
        }).flatten()
    }
}