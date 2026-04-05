use crate::api_types::city::City;

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