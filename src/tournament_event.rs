use std::fmt::Display;

use chrono::{DateTime, Local};
use serenity::all::ScheduledEvent;

use crate::{api_types::Country, lss_api::FabEvent};

pub struct TournamentEvent {
    pub organiser_name: String,
    pub event_name: String,
    pub start_time: DateTime<Local>,
    pub address: String,
    pub description: String,
    country: Country,
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
            event_name: value.nickname.clone(),
            start_time: value.get_start_time_local(),
            address: value.address.clone(),
            description: value.description.clone(),
            country: value.country.clone()
        }
    }
}

impl From<FabEvent> for TournamentEvent {
    fn from(value: FabEvent) -> Self {
        let start_time = value.get_start_time_local();
        Self {
            organiser_name: value.organiser_name,
            event_name: value.nickname,
            start_time: start_time,
            address: value.address,
            description: value.description,
            country: value.country
        }
    }
}

impl Display for TournamentEvent {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{} at {}", self.event_name, self.organiser_name)
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

        format!("| {:<32} | {:<20} | {:18}|", nick, org_name, start_time)
    }
}

pub fn format_fab_events(events: Vec<TournamentEvent>) -> Vec<String> {

    let mut event_list_lines = vec!["```".to_string(), format!("| Events                           | location             | start time        |"), ["="; 80].join("")];

    let mut formated_events: Vec<String> = events
        .iter()
        .map(|event| event.format_event())
        .collect();

    event_list_lines.append(&mut formated_events);
    event_list_lines.push("```".to_string());

    event_list_lines
}