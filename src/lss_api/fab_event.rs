use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::Deserialize;
use crate::api_types::{Country, EventType, Format};

const KIWI_BULLSHIT_MOD: i64 = 10;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FabEvent {
    id: i32,
    pub organiser_name: String,
    pub tournament_type: Format,
    pub nickname: String,
    organiser_store_slug: String,
    pub start_time: DateTime<FixedOffset>,
    pub address: String,
    event_link: Option<String>,
    pub description: String,
    status: String,
    pub format_name: EventType,
    country: Country,
    pub player_cap: Option<i32>,
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
    pub(crate) fn get_start_time_local(&self) -> DateTime<Local> {

        let temp: DateTime<Local> = DateTime::from(self.start_time);
        temp + Duration::hours(KIWI_BULLSHIT_MOD)
    }
}