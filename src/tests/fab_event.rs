use chrono::FixedOffset;
use chrono::TimeZone;
use crate::lss_api::FabEvent;
use crate::structs::Country;

#[tokio::test]
async fn deserialize_mock() {
    
    let mock = r#"{
      "id": 438838,
      "organiser_name": "Midgard Games Oslo",
      "tournament_type": "Pro Tour",
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
    
    let target = FabEvent::new(
        438838,
        "Midgard Games Oslo".to_string(),
        "Pro Tour".to_string(),
        "Midgardgames Armory".to_string(),
        "midgard-games-oslo".to_string(),
        start_time,
        "Ensjøveien 22, 0661 Oslo, Norway".to_string(),
        None,
        "".to_string(),
        "Classic Constructed".to_string(),
        Country::SE,
        None,
        false
    );
    
    assert_eq!(temp, target);
    
}