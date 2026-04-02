use crate::Error;
use std::fmt::Display;
use std::str::FromStr;

use chrono::{DateTime, Duration, FixedOffset, Local};
use reqwest::ClientBuilder;
use serde::Deserialize;

const FAB_API_URL: &str = "https://gem.fabtcg.com/api/v1/locator/events";

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    count: i32,
    next: Option<String>,
    previous: Option<String>,
    pub results: Vec<FabEvent>,
    filters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct FabEvent {
    id: i32,
    organiser_name: String,
    tournament_type: String,
    pub nickname: String,
    organiser_store_slug: String,
    pub start_time: DateTime<FixedOffset>,
    pub address: String,
    event_link: Option<String>,
    pub description: String,
    status: String,
    format_name: String,
    country: String,
    player_cap: Option<i32>,
    live_coverage: bool,
    lat: f64,
    lon: f64,
    distance: f64,
    distance_unit: String,
}

impl FabEvent {
    pub fn get_start_time_local(&self) -> DateTime<Local> {
        let temp: DateTime<Local> = DateTime::from(self.start_time);
        temp + Duration::hours(10)
    }
}

impl Display for FabEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.nickname, self.organiser_name)
    }
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum City {
    Oslo,
    Stavanger,
    Drammen,
    Lillehammer,
    Bodø,
}

impl AsRef<str> for City {
    fn as_ref(&self) -> &str {
        match self {
            City::Oslo => "Oslo, Norge",
            City::Stavanger => "Stavanger, Norge",
            City::Drammen => "Drammen, Norge",
            City::Lillehammer => "Lillehammer, Norge",
            City::Bodø => "Bodø, Norge",
        }
    }
}

impl FromStr for City {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Oslo" | "oslo" => Ok(Self::Oslo),
            "Stavanger" | "stavanger" => Ok(Self::Stavanger),
            "Drammen" | "drammen" => Ok(Self::Drammen),
            "Lillehammer" | "lillehammer" => Ok(Self::Lillehammer),
            "Bodø" | "bodø" => Ok(Self::Bodø),
            x => Err(x.into())
        }
    }
}

pub async fn get_fab_events(city: &City) -> Result<ApiResponse, reqwest::Error> {
    let client = ClientBuilder::new()
        .https_only(true)
        .build()?;

    let query = &[("search", city.as_ref())];
    let request = client
        .get(FAB_API_URL)
        .query(query)
        .build()?;

    let response: ApiResponse = client
        .execute(request)
        .await?
        .json()
        .await?;

    Ok(response)
}

pub fn format_fab_events(response: ApiResponse) -> Result<Vec<String>, reqwest::Error> {

    let mut event_list_lines = vec!["```".to_string(), ["="; 80].join(""), format!("id | Events                           | location             | start time       ")];

    let mut response_lines: Vec<String> = response
        .results
        .iter()
        .enumerate()
        .map(|(i, event)| {
            let mut nick = event.nickname.clone();
            nick.truncate(32);

            let format_string = "%a %d.%m - %H:%M";

            format!(
                "{:2} | {:<32} | {:<20} | {:18}",
                i+1, nick, event.organiser_name, event.get_start_time_local().format(format_string)
            )
        })
        .collect();

    event_list_lines.append(&mut response_lines);
    event_list_lines.push("```".to_string());

    Ok(event_list_lines)
}