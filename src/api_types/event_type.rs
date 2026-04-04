use chrono::Duration;
use serde::Deserialize;
use super::Format;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    #[serde(alias = "Armory Event")]
    #[serde(alias = "armory event")]
    ArmoryEvent,
    Battlegrounds,
    Calling,
    #[serde(alias = "Learn to Play Event")]
    LearnToPlay,
    #[serde(alias = "National Championship")]
    NationalChampionship,
    #[serde(alias = "On Demand")]
    #[serde(alias = "on demand")]
    OnDemand,
    #[serde(alias = "Pro Tour")]
    ProTour,
    Showdown,
    #[serde(alias = "Social Play Event")]
    SocialPlayEvent,
    #[serde(alias = "World Championship Qualifier")]
    WorldChampionshipQualifier,
    #[serde(alias = "World Premiere")]
    WorldPremiere,
    #[serde(other)]
    Unknown
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
            Self::Unknown => "?"
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
            EventType::Battlegrounds |
            EventType::Showdown |
            EventType::NationalChampionship |
            EventType::WorldPremiere |
            EventType::WorldChampionshipQualifier => Some(if cap_based > 0 {cap_based} else {7}),
            EventType::Calling => Some(12),
            EventType::ProTour => Some(14),
            EventType::Unknown => None
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
                        Format::ClassicConstructed("!".to_string())
                        .duration()
                        .checked_mul(8))
                        .map(|(a, b)| a + b),
                _ => format.duration().checked_mul(r),
            }
        }).flatten()
    }
}