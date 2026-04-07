use serde::Deserialize;

#[derive(Debug, Deserialize, poise::ChoiceParameter, PartialEq, Eq, Clone, Copy)]
pub enum Country {
    NO,
    SE,
    DK,
}