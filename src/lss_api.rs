use crate::Error;
use crate::api_types::{City, Country};
use crate::tournament_event::TournamentEvent;

use chrono::{DateTime, Duration, FixedOffset, Local};
use reqwest::{ClientBuilder, IntoUrl};
use serde::Deserialize;

const FAB_API_URL: &str = "https://gem.fabtcg.com/api/v1/locator/events";
const KIWI_BULLSHIT_MOD: i64 = 10;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiResponse {
    count: i32,
    next: Option<String>,
    #[serde(skip)]
    previous: (),
    results: Vec<FabEvent>,
    filters: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FabEvent {
    id: i32,
    pub organiser_name: String,
    tournament_type: String,
    pub nickname: String,
    organiser_store_slug: String,
    pub start_time: DateTime<FixedOffset>,
    pub address: String,
    event_link: Option<String>,
    pub description: String,
    status: String,
    format_name: String,
    pub country: Country,
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

impl ApiResponse {

    pub async fn get_response(city: &City) -> Result<Self, Error> {

        let client: reqwest::Client = ClientBuilder::new().https_only(true).build()?;

        let query = &[("search", city.as_ref())];
        let request = client.get(FAB_API_URL).query(query).build()?;

        let mut response: ApiResponse = client.execute(request).await?.json().await?;

        while let Some(ref url) = response.next {
            
            let next: ApiResponse = Self::from_url(url).await?;
            response.flatten_next(next)?;
        }

        Ok(response)
    }

    async fn from_url<U: IntoUrl>(url: U) -> Result<Self, Error> {

        let client: reqwest::Client = ClientBuilder::new()
            .https_only(true)
            .build()?;

        let request: reqwest::Request = client.get(url).build()?;
        let response: ApiResponse = client.execute(request).await?.json().await?;
        Ok(response)
    }

    fn flatten_next(&mut self, mut other: Self) -> Result<(), Error> {

        self.next = other.next;
        self.results.append(&mut other.results);

        Ok(())
    }

    pub fn get_tournaments(&self) -> Vec<TournamentEvent> {

        let mut r = vec![];

        for event in self.results.iter() {
            r.push(event.into());
        }

        r
    }
}

impl FabEvent {
    pub(crate) fn get_start_time_local(&self) -> DateTime<Local> {

        let temp: DateTime<Local> = DateTime::from(self.start_time);
        temp + Duration::hours(KIWI_BULLSHIT_MOD)
    }
}