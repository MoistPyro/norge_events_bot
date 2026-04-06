use std::str::FromStr;
use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::Deserialize;
use crate::structs::{Country, EventType, Format};

pub const KIWI_BULLSHIT_MOD: i64 = 10;

#[derive(Debug, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct FabEvent {
    id: i32,
    organiser_name: String,
    tournament_type: String,
    nickname: String,
    organiser_store_slug: String,
    start_time: DateTime<FixedOffset>,
    address: String,
    event_link: Option<String>,
    description: String,
    status: String,
    format_name: String,
    country: Country,
    player_cap: Option<i32>,
    live_coverage: bool,
    #[serde(skip)]
    lat: (),
    #[serde(skip)]
    lon: (),
    #[serde(skip)]
    distance: (),
    #[serde(skip)]
    distance_unit: (),
}

impl FabEvent {

    pub fn new(
        id: i32,
        organiser_name: String,
        tournament_type: String,
        nickname: String,
        organiser_store_slug: String,
        start_time: DateTime<FixedOffset>,
        address: String,
        event_link: Option<String>,
        description: String,
        format_name: String,
        country: Country,
        player_cap: Option<i32>,
        live_coverage: bool
    ) -> Self {

        Self {
            id, organiser_name, tournament_type, nickname, organiser_store_slug, start_time, address, event_link, description,
            status: "PLANNED".to_string(),
            format_name, country, player_cap, live_coverage,
            lat: (), lon: (), distance: (), distance_unit: ()
        }
    }

    pub fn org_name(&self) -> &str {
        &self.organiser_name
    }

    pub fn event_nickname(&self) -> &str {
        &self.nickname
    }

    pub fn org_link(&self) -> String {
        "https://fabtcg.com/locator/".to_string() + &self.organiser_store_slug
    }

    pub fn get_start_time_local(&self) -> DateTime<Local> {

        let temp: DateTime<Local> = DateTime::from(self.start_time);
        temp + Duration::hours(KIWI_BULLSHIT_MOD)
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn format(&self) -> Format {
        Format::from_str(&self.format_name).unwrap()
    }

    pub fn event_type(&self) -> EventType {
        EventType::from_str(&self.tournament_type).unwrap()
    }

    pub fn player_cap(&self) -> Option<i32> {
        self.player_cap
    }
}