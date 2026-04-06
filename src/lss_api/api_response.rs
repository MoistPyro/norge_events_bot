use reqwest::IntoUrl;
use serde::Deserialize;
use crate::{Error, structs::City, tournament_event::TournamentEvent};
use super::FabEvent;

const FAB_API_URL: &str = "https://gem.fabtcg.com/api/v1/locator/events";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ApiResponse {
    count: i32,
    next: Option<String>,
    #[serde(skip)]
    previous: (),
    results: Vec<FabEvent>,
    #[serde(skip)]
    filters: (),
}

impl ApiResponse {

    pub fn new(count: i32, next: Option<String>, results: Vec<FabEvent>) -> Self {
        Self { count, next, previous: (), results, filters: () }
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn next(&self) -> &Option<String> {
        &self.next
    }

    pub fn results(&self) -> &Vec<FabEvent> {
        &self.results
    }

    pub async fn get_from_city(city: &impl City) -> Result<Self, Error> {

        let mut page = 1;
        let query = [("search", city.as_ref()), ("mode", "event"), ("page", &format!("{page}"))];
        let mut response: ApiResponse = Self::get_url(FAB_API_URL, &query).await?;
        
        page += 1;

        while let Some(_) = response.next {
            
            let query = [("search", city.as_ref()), ("mode", "event"), ("page", &format!("{page}"))];
            let next: ApiResponse = Self::get_url(FAB_API_URL, &query).await?;
            response.flatten_next(next);

            page += 1;
        }

        Ok(response)
    }

    async fn get_url<U: IntoUrl>(url: U, query: &[(&str, &str)]) -> Result<Self, Error> {

        let client: reqwest::Client = reqwest::ClientBuilder::new().build()?;
        let request: reqwest::Request = client.get(url).query(query).build()?;
        Ok(client.execute(request).await?.json().await?)
    }

    fn flatten_next(&mut self, mut other: Self)  {

        self.next = other.next;
        self.results.append(&mut other.results);
    }

    ///convert from the api type FabEvent, to the more usefull TournamentEvent type
    pub fn get_tournaments(&self) -> Vec<TournamentEvent> {

        // let mut r = vec![];

        // for event in self.results.iter() {
        //     r.push(event.into());
        // }

        // r

        self.results.iter().map(|e| e.into()).collect()
    }
}