use std::fmt::Display;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Duration, Local};
use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, ScheduledEvent};
use tracing::info;
use crate::lss_api::FabEvent;
use crate::structs::{EventType, Format};

const MAX_MESSAGE_LENGTH: usize = 2000;

#[derive(Debug)]
pub struct TournamentEvent {
    pub organiser_name: String,
    pub org_link: String,
    pub event_name: String,
    pub start_time: DateTime<Local>,
    pub address: String,
    pub description: String,
    pub format: Format,
    pub event_type: EventType,
    pub player_cap: Option<i32>,
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
            organiser_name: value.org_name().to_string(),
            org_link: value.org_link(),
            event_name: value.event_nickname().to_string(),
            start_time: value.get_start_time_local(),
            address: value.address().to_string(),
            description: value.description().to_string(),
            format: value.format(),
            event_type: value.event_type(),
            player_cap: value.player_cap(),
        }
    }
}

impl From<FabEvent> for TournamentEvent {
    fn from(value: FabEvent) -> Self {
        let start_time = value.get_start_time_local();
        Self {
            organiser_name: value.org_name().to_string(),
            org_link: value.org_link(),
            event_name: value.event_nickname().to_string(),
            start_time: start_time,
            address: value.address().to_string(),
            description: value.description().to_string(),
            format: value.format(),
            event_type: value.event_type(),
            player_cap: value.player_cap(),
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

/// sorting events order them by start time
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

    pub fn start_time_as_str(&self) -> DelayedFormat<StrftimeItems<'_>>{
        let format_string: &str = "%a %d.%m - %H:%M";
        self.start_time.format(format_string)
    }

    pub fn format_event(&self) -> String {

        let mut nick: String = self.event_name.clone();
        nick.truncate(40);

        let org_name: &str = &self.organiser_name;
        let start_time = self.start_time_as_str();

        format!("{:<40}\ntype: {:<34}\nformat: {:<32}\n{:<20}  {:18}\n", nick, self.event_type.as_ref(), self.format.as_ref(), org_name, start_time)
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

    let mut event_list_lines = vec!["```".to_string()];

    let mut total_characters = 3;
    let num_events = events.len();

    for (i, event) in events.iter().enumerate() {
        let text = event.format_event();
        let text_len = text.len();

        if text_len + total_characters <= MAX_MESSAGE_LENGTH - 22 {
            event_list_lines.push(text);
            total_characters += text_len + 1;
        } else {
            event_list_lines.push(format!("{:>3} events hidden.", num_events - i));
            break;
        }
    }

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