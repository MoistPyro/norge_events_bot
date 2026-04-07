use chrono::Duration;
use chrono::FixedOffset;
use chrono::TimeZone;
use chrono::Local;
use chrono::DateTime;

use crate::lss_api::FabEvent;
use crate::structs::Country;
use crate::lss_api::fab_event::KIWI_BULLSHIT_MOD;
use crate::structs::EventType;
use crate::structs::Format;
use crate::tournament_event::TournamentEvent;
use crate::tournament_event::format_fab_events;

fn mock_tournament_event() -> TournamentEvent {
    let start_time: DateTime<Local> = DateTime::from(FixedOffset::east_opt(12 * 3600)
        .unwrap()
        .with_ymd_and_hms(2026, 4, 07, 17, 0, 0)
        .unwrap())
        + Duration::hours(KIWI_BULLSHIT_MOD);

    TournamentEvent {
        organiser_name: "Midgard Games Oslo".to_string(),
        org_link: "https://fabtcg.com/locator/midgard-games-oslo".to_string(),
        event_name: "Midgardgames Armory".to_string(),
        start_time,
        address: "Ensjøveien 22, 0661 Oslo, Norway".to_string(),
        description: "".to_string(),
        format: Format::ClassicConstructed,
        event_type: EventType::ProTour,
        player_cap: None
    }
}

#[test]
fn test_format_event() {

    let temp = mock_tournament_event();
    let temp_format = temp.format_event();
    
    const EXPECTED: &str = "Midgardgames Armory                     \ntype: Pro Tour                          \nformat: Classic Constructed             \nMidgard Games Oslo    Tue 07.04 - 17:00 \n";

    assert_eq!(temp_format, EXPECTED)
}

#[test]
fn test_format_multiple() {

    let events = vec![mock_tournament_event(), mock_tournament_event()];

    let formating = format_fab_events(events);
    let content = formating.content.unwrap();

    const EXPECTED: &str = "```\nMidgardgames Armory                     \ntype: Pro Tour                          \nformat: Classic Constructed             \nMidgard Games Oslo    Tue 07.04 - 17:00 \n\nMidgardgames Armory                     \ntype: Pro Tour                          \nformat: Classic Constructed             \nMidgard Games Oslo    Tue 07.04 - 17:00 \n\n```";

    assert_eq!(content, EXPECTED)
}

#[tokio::test]
async fn test_from_fab_event() {
    
    let start_time = FixedOffset::east_opt(12 * 3600).unwrap()
        .with_ymd_and_hms(2026, 4, 07, 17, 0, 0).unwrap();
        
    let temp = FabEvent::new(
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
    
    let target = mock_tournament_event();
    
    let temp = TournamentEvent::from(temp);
    
    assert_eq!(temp, target)
}
