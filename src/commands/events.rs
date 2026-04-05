use poise::CreateReply;

use crate::{Context, Error, tournament_event};
use crate::lss_api::ApiResponse;
use crate::api_types::EveryCity;
use super::error_hander;

/// Get a list of Flesh and Blood events in the choosen city.
#[poise::command(
    slash_command,
    prefix_command,
    on_error = "error_hander"
)]
pub async fn events(ctx: Context<'_>, #[description = "city:"] city: EveryCity) -> Result<(), Error> {

    let response: ApiResponse = ApiResponse::get_from_city(&city).await?;
    let fab_events: Vec<tournament_event::TournamentEvent> = response.get_tournaments();
    let message: CreateReply = tournament_event::format_fab_events(fab_events);

    ctx.send(message).await?;
    Ok(())
}