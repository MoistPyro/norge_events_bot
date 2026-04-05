use reqwest::IntoUrl;
use serde::Deserialize;
use crate::{Error, api_types::City, tournament_event::TournamentEvent};
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

    pub async fn get_from_city(city: &City) -> Result<Self, Error> {

        let mut page = 1;
        let query = [("search", city.as_ref()), ("page", &format!("{page}"))];
        let mut response: ApiResponse = Self::get_url(FAB_API_URL, &query).await?;
        
        page += 1;

        while let Some(_) = response.next {
            
            let query = [("search", city.as_ref()), ("page", &format!("{page}"))];
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

#[cfg(test)]
mod test {
    use super::*;

    ///this passes if it runs without error.
    #[tokio::test]
    async fn it_works() {

        let temp = ApiResponse::get_from_city(&City::Oslo).await.expect("expected a well formed response");
    }
}