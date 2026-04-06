use std::fmt::Display;
use std::str::FromStr;
use chrono::{DateTime, Duration, Local};
use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, ScheduledEvent};
use tracing::info;
use crate::lss_api::FabEvent;
use crate::api_types::{EventType, Format};

#[derive(Debug)]
pub struct TournamentEvent {
    pub organiser_name: String,
    org_link: String,
    pub event_name: String,
    pub start_time: DateTime<Local>,
    pub address: String,
    pub description: String,
    format: Format,
    event_type: EventType,
    player_cap: Option<i32>,
}

impl PartialEq<ScheduledEvent> for TournamentEvent {
    fn eq(&self, other: &ScheduledEvent) -> bool {

        self.start_time.naive_utc() == other.start_time.naive_utc() && self.event_name == other.name
    }
}

impl PartialEq<&ScheduledEvent> for TournamentEvent {
    fn eq(&self, other: &&ScheduledEvent) -> bool {

        self.start_time.naive_utc() == other.start_time.naive_utc() && self.event_name == other.name
    }
}

impl From<&FabEvent> for TournamentEvent {
    fn from(value: &FabEvent) -> Self {
        Self {
            organiser_name: value.organiser_name.clone(),
            org_link: "https://fabtcg.com/locator/".to_owned() + &value.organiser_store_slug,
            event_name: value.nickname.clone(),
            start_time: value.get_start_time_local(),
            address: value.address.clone(),
            description: value.description.clone(),
            format: Format::from_str(&value.format_name).unwrap(),
            event_type: EventType::from_str(&value.tournament_type).unwrap(),
            player_cap: value.player_cap,
        }
    }
}

impl From<FabEvent> for TournamentEvent {
    fn from(value: FabEvent) -> Self {
        let start_time = value.get_start_time_local();
        Self {
            organiser_name: value.organiser_name,
            org_link: "https://fabtcg.com/locator/".to_owned() + &value.organiser_store_slug,
            event_name: value.nickname,
            start_time: start_time,
            address: value.address,
            description: value.description,
            format: Format::from_str(&value.format_name).unwrap(),
            event_type: EventType::from_str(&value.tournament_type).unwrap(),
            player_cap: value.player_cap,
        }
    }
}

impl Display for TournamentEvent {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{} at {}", self.event_name, self.organiser_name)
    }
}

impl PartialEq for TournamentEvent {
    fn eq(&self, other: &Self) -> bool {
        self.organiser_name == other.organiser_name &&
        self.event_name == other.event_name &&
        self.start_time == other.start_time &&
        self.address == other.address  &&
        self.format == other.format &&
        self.event_type == other.event_type
    }
}

impl Eq for TournamentEvent {}

impl PartialOrd for TournamentEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {

        self.start_time.partial_cmp(&other.start_time)
    }
}

impl Ord for TournamentEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_time.cmp(&other.start_time)
    }
}

impl TournamentEvent {
    pub fn is_already_imported(&self, active_events: &Vec<ScheduledEvent>) -> bool {

        active_events.iter()
            .find(|e| self.eq(e))
            .is_some()
    }

    fn format_event(&self) -> String {

        let mut nick: String = self.event_name.clone();
        nick.truncate(32);

        let org_name: &str = &self.organiser_name;

        let format_string: &str = "%a %d.%m - %H:%M";
        let start_time = self.start_time.format(format_string);

        format!("| {:<32} | {:<20} | {:18}| {} {}", nick, org_name, start_time, self.format.as_ref(), self.event_type.as_ref())
    }

    ///returns the approximate duration of the event, or two hours.
    pub fn calculate_duration(&self) -> Duration {
        self.event_type.duration(&self.format, self.player_cap).unwrap_or(Duration::hours(2))
    }

    pub fn _make_embed(&self) -> CreateEmbed {

        let format_string: &str = "%a %d.%m - %H:%M";
        let start_time = self.start_time.format(format_string);

        let fields: Vec<(&str, String, bool)> = vec![
            ("start time:", format!("{}", start_time), true),
            ("address:", self.address.clone(), false),
            ("format:", self.format.as_ref().to_owned(), true),
            ("event type:", self.event_type.as_ref().to_owned(), false),
        ];

        CreateEmbed::new()
            .colour(Colour::DARK_MAGENTA)
            .title(format!("{self}"))
            .url(&self.org_link)
            .fields(fields)
    }
}

pub fn format_fab_events(events: Vec<TournamentEvent>) -> CreateReply {

    let mut event_list_lines = vec!["```".to_string(), format!("| Events                           | location             | start time        |"), ["="; 80].join("")];

    let mut formated_events: Vec<String> = events
        .iter()
        .map(|event| event.format_event())
        .collect();

    event_list_lines.append(&mut formated_events);
    event_list_lines.push("```".to_string());

    let content = event_list_lines.join("\n");
    CreateReply::default().content(content)
}

pub fn _format_embeds(events: Vec<TournamentEvent>) -> CreateReply {

    let mut reply = CreateReply::default();

    for event in events.iter().take(10) {
        info!("an embed");
        reply = reply.embed(event._make_embed());
    }

    //TODO: make fancy logic for finding relevant events

    reply
}

#[cfg(test)]
mod test {
    use chrono::FixedOffset;
    use chrono::TimeZone;
    use chrono::Local;
    use chrono::DateTime;

    use crate::api_types::Country;
    use crate::lss_api::fab_event::KIWI_BULLSHIT_MOD;

    use super::*;

    #[tokio::test]
    async fn test_from_fab_event() {

        let start_time = FixedOffset::east_opt(12 * 3600).unwrap()
            .with_ymd_and_hms(2026, 4, 07, 17, 0, 0).unwrap();

        let other_time: DateTime<Local> = DateTime::from(start_time) + Duration::hours(KIWI_BULLSHIT_MOD);

        let temp = FabEvent {
            id: 438838,
            organiser_name: "Midgard Games Oslo".to_string(),
            tournament_type: "Pro Tour".to_string(),
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

        let target = TournamentEvent {
            organiser_name: "Midgard Games Oslo".to_string(),
            org_link: "https://fabtcg.com/locator/midgard-games-oslo".to_string(),
            event_name: "Midgardgames Armory".to_string(),
            start_time: other_time,
            address: "Ensjøveien 22, 0661 Oslo, Norway".to_string(),
            description: "".to_string(),
            format: Format::ClassicConstructed,
            event_type: EventType::ProTour,
            player_cap: None
        };

        let temp = TournamentEvent::from(temp);

        assert_eq!(temp, target)
    }
}