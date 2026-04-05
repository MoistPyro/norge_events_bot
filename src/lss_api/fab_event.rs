use chrono::{DateTime, Duration, FixedOffset, Local};
use serde::Deserialize;
use crate::api_types::Country;

const KIWI_BULLSHIT_MOD: i64 = 10;

#[derive(Debug, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct FabEvent {
    id: i32,
    pub organiser_name: String,
    pub tournament_type: String,
    pub nickname: String,
    pub organiser_store_slug: String,
    pub start_time: DateTime<FixedOffset>,
    pub address: String,
    event_link: Option<String>,
    pub description: String,
    status: String,
    pub format_name: String,
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

#[cfg(test)]
mod test {

    use chrono::FixedOffset;
    use chrono::TimeZone;
    use super::*;

    #[tokio::test]
    async fn deserialize_mock() {
        
        let mock = r#"{
      "id": 438838,
      "organiser_name": "Midgard Games Oslo",
      "tournament_type": "Armory Event",
      "nickname": "Midgardgames Armory",
      "organiser_store_slug": "midgard-games-oslo",
      "start_time": "2026-04-07T17:00:00+12:00",
      "address": "Ensjøveien 22, 0661 Oslo, Norway",
      "event_link": null,
      "description": "",
      "status": "PLANNED",
      "format_name": "Classic Constructed",
      "country": "SE",
      "player_cap": null,
      "live_coverage": false,
      "lat": 59.9133941,
      "lon": 10.7903995,
      "distance": 3.3,
      "distance_unit": "km"
    }"#;

    let temp: FabEvent = serde_json::from_str(&mock).unwrap();

    let start_time = FixedOffset::east_opt(12 * 3600).unwrap()
        .with_ymd_and_hms(2026, 4, 07, 17, 0, 0).unwrap();

    let target = FabEvent{
        id: 438838,
        organiser_name: "Midgard Games Oslo".to_string(),
        tournament_type: "Armory Event".to_string(),
        nickname: "Midgardgames Armory".to_string(),
        organiser_store_slug: "midgard-games-oslo".to_string(),
        start_time: start_time,
        address: "Ensjøveien 22, 0661 Oslo, Norway".to_string(),
        event_link: None,
        description: "".to_string(),
        status: "PLANNED".to_string(),
        format_name: "Classic Constructed".to_string(),
        country: Country::SE,
        player_cap: None,
        live_coverage: false,
        lat: (),
        lon: (),
        distance: (),
        distance_unit: ()
    };

    assert_eq!(temp, target);

    }
}