use crate::Error;
use std::fmt::Display;
use std::str::FromStr;

use chrono::Duration;
use reqwest::ClientBuilder;
use serde::Deserialize;
use chrono::DateTime;
use chrono::Utc;

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
    pub start_time: DateTime<Utc>,
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

impl Display for FabEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.nickname, self.organiser_name)
    }
}

impl FabEvent {
    fn fix_time(&self) -> DateTime<Utc> {
        self.start_time + Duration::hours(12)
    }
}

pub enum City {
    Oslo,
    Stavanger,
    Drammen,
}

impl AsRef<str> for City {
    fn as_ref(&self) -> &str {
        match self {
            City::Oslo => "Oslo, Norge",
            City::Stavanger => "Stavanger, Norge",
            City::Drammen => "Drammen, Norge",
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

    client.execute(request).await?
        .json().await
}

pub async fn format_fab_events(city: &City) -> Result<Vec<String>, reqwest::Error> {
    let response = get_fab_events(city).await?;

    let mut event_list_lines = vec![format!("Events in {}:", city.as_ref())];
    let mut response_lines: Vec<String> = response
        .results
        .iter()
        .enumerate()
        .map(|(i, event)| {
            let mut nick = event.nickname.clone();
            nick.truncate(34);

            let right_time = event.fix_time();
            let format_string = "%A %d.%m - %H:%M";

            format!(
                "{}: {:<35} at {}\r\nstart time: {}",
                i+1, nick, event.organiser_name, right_time.format(format_string)
            )
        })
        .collect();

    event_list_lines.append(&mut response_lines);

    Ok(event_list_lines)
}